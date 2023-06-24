use anyhow::{anyhow, bail, Context, Result};
use cursive::utils::ProgressReader;
use log::info;
use nix::dir::Dir;
use nix::errno::Errno;
use nix::fcntl::{FallocateFlags, OFlag};
use nix::mount;
use nix::sys::stat::Mode;
use nix::unistd::{chroot, fchdir, sync};
use std::ffi::OsStr;
use std::fmt::Debug;
use std::io::{prelude::*, SeekFrom, Write};
use std::os::unix::io::AsRawFd;
use std::os::unix::prelude::{OsStrExt, PermissionsExt};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{fs::File, path::Path};
use sysinfo::{System, SystemExt};

use crate::disks::{fstab_entries, is_efi_booted, Partition};
use crate::network;
use crate::parser::{list_mounts, list_zoneinfo, locale_names};

const BIND_MOUNTS: &[&str] = &["/dev", "/proc", "/sys", "/run/udev"];
const EFIVARS_PATH: &str = "/sys/firmware/efi/efivars";
const BUNDLED_LOCALE_GEN: &[u8] = include_bytes!("../res/locale.gen");
const SYSTEM_LOCALE_GEN_PATH: &str = "/etc/locale.gen";
const SYSTEM_ZONEINFO1970_PATH: &str = "/usr/share/zoneinfo/zone1970.tab";
const BUNDLED_ZONEINFO_LIST: &[u8] = include_bytes!("../res/zone1970.tab");

fn run_command<I, S>(command: &str, args: I) -> Result<()>
where
    I: IntoIterator<Item = S> + Debug,
    S: AsRef<OsStr>,
{
    let cmd_str = format!("{command} {args:?}");
    info!("Running {}", cmd_str);

    let cmd = Command::new(command).args(args).output()?;

    if !cmd.status.success() {
        return Err(anyhow!(
            "Run {} failed!\n\n{}",
            cmd_str,
            String::from_utf8_lossy(&cmd.stderr)
        ));
    }

    info!("Run {} Successfully!", cmd_str);

    Ok(())
}

fn no_need_to_run_info(s: &str, str_is_retro: bool) {
    if str_is_retro {
        info!("Retro system no need to run {}", s);
    } else {
        info!("Non retro system no need to run {}", s);
    }
}

#[derive(Debug)]
pub enum ExtractFileType {
    Tar,
    Squashfs,
}

fn read_system_locale_list() -> Result<Vec<u8>> {
    let mut f = std::fs::File::open(SYSTEM_LOCALE_GEN_PATH)?;
    let mut data: Vec<u8> = Vec::new();
    data.reserve(8800);
    f.read_to_end(&mut data)?;

    Ok(data)
}

/// Get the list of available locales
pub fn get_locale_list() -> Result<Vec<String>> {
    let data = read_system_locale_list().unwrap_or_else(|_| BUNDLED_LOCALE_GEN.to_vec());
    let names = locale_names(&data)
        .map_err(|_| anyhow!("Installer failed to gather available locales."))?;
    let names = names.1.into_iter().map(|x| x.to_string()).collect();

    Ok(names)
}

fn read_system_zoneinfo_list() -> Result<Vec<u8>> {
    let mut f = std::fs::File::open(SYSTEM_ZONEINFO1970_PATH)?;
    let mut data: Vec<u8> = Vec::new();
    data.reserve(8800);
    f.read_to_end(&mut data)?;

    Ok(data)
}

/// Get the list of available timezone
pub fn get_zoneinfo_list() -> Result<Vec<String>> {
    let data = read_system_zoneinfo_list().unwrap_or_else(|_| BUNDLED_ZONEINFO_LIST.to_vec());
    let mut zoneinfo_list = list_zoneinfo(&data)
        .map_err(|_| anyhow!("Installer failed to gather available timezones."))?
        .1;

    if zoneinfo_list.is_empty() {
        return Err(anyhow!(
            "Installer could not parse the zoneinfo database! Empty timezone data (tzdata)?"
        ));
    }

    zoneinfo_list.sort();
    zoneinfo_list.insert(0, "UTC".to_string());

    Ok(zoneinfo_list)
}

/// Extract the given .tar.xz stream and preserve all the file attributes
fn extract_tar_xz<P: AsRef<Path>, R: Read>(reader: R, path: P) -> Result<()> {
    let decompress = xz2::read::XzDecoder::new(reader);
    let mut tar_processor = tar::Archive::new(decompress);
    tar_processor.set_unpack_xattrs(true);
    tar_processor.set_preserve_permissions(true);
    tar_processor.set_preserve_ownerships(true);
    tar_processor.unpack(path)?;

    Ok(())
}

/// Extract the .squashfs and callback download progress
fn extract_squashfs<P: AsRef<Path>>(
    file_size: f64,
    archive: P,
    path: P,
    counter: cursive::utils::Counter,
) -> Result<()> {
    let mut sys = System::new_all();
    sys.refresh_memory();
    let total_memory = sys.total_memory() / 1024 / 1024 / 1024;

    let limit_thread = if total_memory <= 2 { Some(1) } else { None };

    distinst_squashfs::extract(archive, path, limit_thread, move |count| {
        counter.set((file_size * count as f64 / 100.0) as usize);
    })?;

    Ok(())
}

/// Extract .tar.xz or .squashfs
pub fn extract_file(
    file_size: f64,
    url: String,
    archive_path: &Path,
    extract_path: &Path,
    counter: cursive::utils::Counter,
) -> Result<()> {
    let extract_file_type = if url.ends_with(".squashfs") {
        ExtractFileType::Squashfs
    } else if url.ends_with(".tar.xz") {
        ExtractFileType::Tar
    } else {
        return Err(anyhow!("Unsupport format!"));
    };

    match extract_file_type {
        ExtractFileType::Tar => extract_tar_xz(
            ProgressReader::new(counter, std::fs::File::open(archive_path)?),
            extract_path,
        ),
        ExtractFileType::Squashfs => {
            extract_squashfs(file_size, archive_path, extract_path, counter)
        }
    }
}

pub fn auto_mount_root_path(tmp_path: &Path, partition: &Partition) -> Result<PathBuf> {
    mount_root_path(partition, tmp_path)?;

    Ok(tmp_path.to_path_buf())
}

/// Sync the filesystem and then reboot IMMEDIATELY (ignores init)
pub fn sync_and_reboot() -> Result<()> {
    sync();
    run_command("systemctl", ["reboot"])?;

    Ok(())
}

/// Mount the filesystem
pub fn mount_root_path(partition: &Partition, target: &Path) -> Result<()> {
    if partition.fs_type.is_none() || partition.path.is_none() {
        return Err(anyhow!(
            "Installer failed to determine user-specified partition."
        ));
    }
    let source = partition.path.as_ref();
    let mut fs_type = partition.fs_type.as_ref().unwrap().as_str();
    if fs_type.starts_with("fat") {
        fs_type = "vfat";
    }
    // FIXME: due to an issue in `nix` and `libc`, `MS_LAZYTIME` is not supported atm
    mount::mount(
        source,
        target,
        Some(fs_type),
        mount::MsFlags::empty(),
        None::<&str>,
    )?;

    Ok(())
}

/// Gen fstab to /etc/fstab
pub fn genfstab_to_file(partition: &Partition, root_path: &Path, mount_path: &Path) -> Result<()> {
    if cfg!(debug_assertions) {
        return Ok(());
    }
    let fs_type = partition.fs_type.as_ref().ok_or_else(|| {
        anyhow!("Installer failed to detect filesystem type for the specified partition.")
    })?;
    let s = fstab_entries(partition.path.as_ref(), fs_type, Some(mount_path))?;
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(root_path.join("etc/fstab"))?;
    f.write_all(s.as_bytes())?;

    Ok(())
}

/// Unmount the filesystem given at `root` and then do a sync
pub fn umount_root_path(root: &Path) -> Result<()> {
    mount::umount2(root, mount::MntFlags::MNT_DETACH)?;
    sync();

    Ok(())
}

/// Get the open file descriptor to the specified path
pub fn get_dir_fd<P: nix::NixPath>(path: P) -> Result<Dir> {
    let fd = Dir::open(
        &path,
        OFlag::O_RDONLY | OFlag::O_DIRECTORY | OFlag::O_NONBLOCK,
        Mode::empty(),
    )?;

    Ok(fd)
}

/// Escape the chroot context using the previously obtained `root_fd` as a trampoline
pub fn escape_chroot(root_fd: i32) -> Result<()> {
    fchdir(root_fd)?;
    chroot(".")?;
    std::env::set_current_dir("/")?; // reset cwd (on host)

    info!("Escaped chroot environment");

    Ok(())
}

/// Setup all the necessary bind mounts
pub fn setup_bind_mounts(root: &Path) -> Result<()> {
    for mount in BIND_MOUNTS {
        let mut root = root.to_owned();
        root.push(&mount[1..]);
        std::fs::create_dir_all(root.clone())?;
        mount::mount(
            Some(*mount),
            &root,
            None::<&str>,
            mount::MsFlags::MS_BIND,
            None::<&str>,
        )?;
    }

    if is_efi_booted() {
        let root = root.join(&EFIVARS_PATH[1..]);
        std::fs::create_dir_all(&root)?;
        mount::mount(
            Some(EFIVARS_PATH),
            &root,
            None::<&str>,
            mount::MsFlags::MS_BIND,
            None::<&str>,
        )?;
    }

    Ok(())
}

/// Remove bind mounts
/// Note: This function should be called outside of the chroot context
pub fn remove_bind_mounts(root: &Path) -> Result<()> {
    for mount in BIND_MOUNTS {
        let mut root = root.to_owned();
        root.push(&mount[1..]);
        mount::umount2(&root, mount::MntFlags::MNT_DETACH)?;
    }

    Ok(())
}

/// Setup bind mounts and chroot into the guest system
/// Warning: This will make the program trapped in the new root directory
pub fn dive_into_guest(root: &Path) -> Result<()> {
    setup_bind_mounts(root)?;
    chroot(root)?;
    std::env::set_current_dir("/")?; // jump to the root directory after chroot

    Ok(())
}

/// Runs dracut
/// Must be used in a chroot context
#[cfg(not(feature = "is_retro"))]
pub fn execute_dracut() -> Result<()> {
    let cmd = "/usr/bin/update-initramfs";
    run_command(cmd, &[] as &[&str])?;

    Ok(())
}

/// Runs dracut (dummy function for retro mode)
/// Must be used in a chroot context
#[cfg(feature = "is_retro")]
pub fn execute_dracut() -> Result<()> {
    no_need_to_run_info("dracut", true);

    Ok(())
}

/// Runs ssh-keygen -A (dummy function for non-retro mode)
/// Must be used in a chroot context
#[cfg(not(feature = "is_retro"))]
pub fn gen_ssh_key() -> Result<()> {
    no_need_to_run_info("ssh-keygen", false);

    Ok(())
}

/// Runs ssh-keygen -A
/// Must be used in a chroot context
#[cfg(feature = "is_retro")]
pub fn gen_ssh_key() -> Result<()> {
    run_command("ssh-keygen", &["-A"])?;

    Ok(())
}

/// Sets hostname in the guest environment
/// Must be used in a chroot context
pub fn set_hostname(name: &str) -> Result<()> {
    let mut f = File::create("/etc/hostname")?;

    Ok(f.write_all(name.as_bytes())?)
}

/// Sets locale in the guest environment
/// Must be used in a chroot context
pub fn set_locale(locale: &str) -> Result<()> {
    let mut f = File::create("/etc/locale.conf")?;
    f.write_all(b"LANG=")?;

    Ok(f.write_all(locale.as_bytes())?)
}

/// Sets zoneinfo in the guest environment
/// Must be used in a chroot context
pub fn set_zoneinfo(zone: &str) -> Result<()> {
    if Path::new("/etc/localtime").exists() {
        std::fs::remove_file("/etc/localtime")?;
    }

    std::os::unix::fs::symlink(format!("/usr/share/zoneinfo/{zone}"), "/etc/localtime")?;

    Ok(())
}

/// Sets utc/rtc time in the guest environment
/// Must be used in a chroot context
pub fn set_hwclock_tc(utc: bool) -> Result<()> {
    let adjtime_file = std::fs::File::open("/etc/adjtime");
    let status_is_rtc = if let Ok(mut adjtime_file) = adjtime_file {
        let mut buf = String::new();
        adjtime_file.read_to_string(&mut buf)?;
        let line: Vec<&str> = buf.split('\n').collect();
        if line.len() < 3 || line.get(2) == Some(&"UTC") {
            false
        } else {
            line[2] == "LOCAL"
        }
    } else {
        false
    };

    info!("Status is rtc: {}", status_is_rtc);
    if utc {
        if !status_is_rtc {
            return Ok(());
        } else {
            run_command("hwclock", ["-wu"])?;
        }
    } else if status_is_rtc {
        return Ok(());
    } else {
        run_command("hwclock", ["-wl"])?;
    }

    Ok(())
}

/// Adds a new normal user to the guest environment
/// Must be used in a chroot context
pub fn add_new_user(name: &str, password: &str) -> Result<()> {
    run_command("useradd", ["-m", "-s", "/bin/bash", name])?;
    run_command("usermod", ["-aG", "audio,cdrom,video,wheel,plugdev", name])?;

    info!("Running chpasswd ...");
    let command = Command::new("chpasswd").stdin(Stdio::piped()).spawn()?;

    let mut stdin = command.stdin.ok_or_else(|| {
        anyhow!("Installer can not get your stdin! please restart your environment")
    })?;

    stdin.write_all(format!("{name}:{password}\n").as_bytes())?;
    stdin.flush()?;
    info!("Running chpasswd successfully");

    Ok(())
}

struct Passwd {
    username: String,
    time: String,
    uid: String,
    gid: String,
    fullname: String,
    home: String,
    login_shell: String,
}

fn set_full_name(full_name: &str, username: &str, passwd: Vec<&str>) -> Result<String> {
    if full_name.contains(':') || full_name.contains('\n') {
        bail!("Illegal full name");
    }

    let mut v = Vec::new();
    for i in passwd {
        if i.trim().is_empty() {
            continue;
        }

        let mut i_split = i.split(':');
        // 用户名
        let i_username = i_split
            .next()
            .context(format!("Can not get username for entry: {}", i))?;
        // 过期时间
        let time = i_split
            .next()
            .context(format!("Can not get time for entry: {}", i))?;
        // uid
        let uid = i_split
            .next()
            .context(format!("Can not get uid for entry: {}", i))?;
        // gid
        let gid = i_split
            .next()
            .context(format!("Can not get gid for entry: {}", i))?;
        // fullname
        let i_fullname = i_split
            .next()
            .context(format!("Can not get fullname for entry: {}", i))?;
        // 家路径
        let home_directory = i_split
            .next()
            .context(format!("Can not get home dir for entry: {}", i))?;
        // Login shell
        let login_shell = i_split
            .next()
            .context(format!("Can not get login_shell for entry: {}", i))?;

        v.push(Passwd {
            username: i_username.to_owned(),
            time: time.to_owned(),
            uid: uid.to_owned(),
            gid: gid.to_owned(),
            fullname: i_fullname.to_owned(),
            home: home_directory.to_owned(),
            login_shell: login_shell.to_string(),
        })
    }

    let index = v.iter().position(|x| x.username == username);

    if index.is_none() {
        bail!("Unknown Error");
    }

    v[index.unwrap()].fullname = full_name.to_owned();

    let mut s = String::new();

    for i in v {
        let entry = vec![
            i.username,
            i.time,
            i.uid,
            i.gid,
            i.fullname,
            i.home,
            i.login_shell,
        ]
        .join(":")
            + "\n";
        s.push_str(&entry);
    }

    Ok(s)
}

/// Sets Fullname
/// Must be used in a chroot context
pub fn passwd_set_fullname(full_name: &str, username: &str) -> Result<()> {
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .read(true)
        .open("/etc/passwd")?;

    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let passwd = buf.split('\n').collect::<Vec<_>>();

    let s = set_full_name(full_name, username, passwd)?;
    f.seek(SeekFrom::Start(0))?;

    f.write_all(s.as_bytes())?;

    Ok(())
}

/// Runs grub-install and grub-mkconfig
/// Must be used in a chroot context
#[cfg(not(target_arch = "powerpc64"))]
pub fn execute_grub_install(mbr_dev: Option<&PathBuf>) -> Result<()> {
    let mut grub_install_args = vec![];

    if let Some(mbr_dev) = mbr_dev {
        grub_install_args.push("--target=i386-pc");
        grub_install_args.push(
            mbr_dev
                .to_str()
                .ok_or_else(|| anyhow!("Can not mbr_dev path to str!"))?,
        );
    } else {
        let (target, is_efi) = match network::get_arch_name() {
            Some("amd64") => ("--target=x86_64-efi", true),
            Some("arm64") => ("--target=arm64-efi", true),
            Some("riscv64") => ("--target=riscv64-efi", true),
            _ => {
                info!("This architecture does not support grub");
                return Ok(());
            }
        };
        grub_install_args.push("--bootloader-id=AOSC OS");
        grub_install_args.push(target);
        if is_efi {
            grub_install_args.push("--efi-directory=/efi");
        }
    };

    run_command("grub-install", &grub_install_args)?;
    run_command("grub-mkconfig", ["-o", "/boot/grub/grub.cfg"])?;

    Ok(())
}

#[cfg(target_arch = "powerpc64")]
pub fn execute_grub_install(_mbr_dev: Option<&PathBuf>) -> Result<()> {
    use std::io::BufReader;

    let target = network::get_arch_name();

    let cpuinfo = std::fs::File::open("/proc/cpuinfo")?;
    let r = BufReader::new(cpuinfo);

    let find = r.lines().flatten().find(|x| x.starts_with("firmware"));

    let needs_install = if let Some(find) = find {
        let s = find.split(':').nth(1).map(|x| x.trim());

        s != Some("OPAL")
    } else {
        true
    };

    let install_args = match target {
        Some("ppc64el") | Some("ppc64") | Some("powerpc") => "--target=powerpc-ieee1275",
        _ => {
            info!("This architecture does not support grub");
            return Ok(());
        }
    };

    if needs_install {
        run_command("grub-install", &[install_args])?;
    }

    run_command("grub-mkconfig", ["-o", "/boot/grub/grub.cfg"])?;

    Ok(())
}

pub fn prepare_try_umount() -> Result<()> {
    let mut mounts = std::fs::File::open("/proc/mounts")?;
    let mut buf = Vec::new();
    mounts.read_to_end(&mut buf)?;

    let mounts = list_mounts(&buf)
        .map_err(|e| anyhow!("Failed to get mounts, {}", e))?
        .1;

    let dk_mounts = mounts
        .iter()
        .filter(|(_, mount_path)| mount_path.starts_with("/tmp/.dkmount"));

    for (_, mount_path) in dk_mounts {
        umount_root_path(Path::new(mount_path)).ok();
    }

    Ok(())
}

pub fn log_system_info() {
    let sys = System::new_all();

    info!("Deploykit version: {}", env!("CARGO_PKG_VERSION"));
    info!(
        "OS: {:?}",
        sys.name()
            .and_then(|x| sys.os_version().map(|y| format!("{x} {y}")))
    );
    info!("Kernel: {:?}", sys.kernel_version());
    info!("CPU: {:?}", sys.cpus().first());
    info!(
        "Memory: {:?}, Usage: {:?}",
        sys.total_memory(),
        sys.used_memory()
    );
}

/// Create swapfile
pub fn create_swapfile(size: f64, use_swap: bool, tempdir: &Path) -> Result<()> {
    if !use_swap {
        return Ok(());
    }

    let swap_path = tempdir.join("swapfile");

    info!("Creating swapfile");
    let mut swapfile = std::fs::File::create(&swap_path)?;

    let res = unsafe {
        libc::fallocate64(
            swapfile.as_raw_fd(),
            FallocateFlags::empty().bits(),
            0,
            size as i64,
        )
    };

    Errno::result(res).map(drop)?;

    swapfile.flush()?;

    info!("Set swapfile permission as 600");
    std::fs::set_permissions(&swap_path, std::fs::Permissions::from_mode(0o600))?;

    run_command("mkswap", [&swap_path])?;
    run_command("swapon", [swap_path]).ok();

    Ok(())
}

pub fn swapoff(tempdir: &Path) {
    run_command("swapoff", [tempdir.join("swapfile")]).ok();
}

/// Must be used in a chroot context
pub fn write_swap_entry_to_fstab() -> Result<()> {
    let s = "/swapfile none swap defaults,nofail 0 0\n";
    let mut fstab = std::fs::OpenOptions::new()
        .append(true)
        .open("/etc/fstab")?;
    fstab.write_all(s.as_bytes())?;

    Ok(())
}

/// Run umount -R
pub fn umount_all(mount_path: &Path, root_fd: i32) {
    info!("Cleaning up mount path ...");

    escape_chroot(root_fd).ok();
    let efi_path = mount_path.join("efi");
    if is_efi_booted() {
        umount_root_path(&efi_path).ok();
    }
    swapoff(mount_path);
    umount_root_path(mount_path).ok();
}

pub fn is_valid_hostname(hostname: &str) -> bool {
    if hostname.is_empty() || hostname.starts_with('-') {
        return false;
    }
    for c in hostname.as_bytes() {
        if c.is_ascii_alphanumeric() || *c == b'-' {
            continue;
        } else {
            return false;
        }
    }

    true
}

pub fn is_acceptable_username(username: &str) -> bool {
    if username.is_empty() {
        return false;
    }

    if username == "root" {
        return false;
    }

    for (i, c) in username.as_bytes().iter().enumerate() {
        if i == 0 {
            if !c.is_ascii_lowercase() {
                return false;
            }
        } else if !c.is_ascii_lowercase() && !c.is_ascii_digit() {
            return false;
        }
    }

    true
}

#[test]
fn test_hostname_validation() {
    assert!(is_valid_hostname("foo"));
    assert!(is_valid_hostname("foo-2e10"));
    assert!(is_valid_hostname("jeffbai-device"));
    assert!(!is_valid_hostname("invalid_host"));
    assert!(!is_valid_hostname("-invalid"));
    assert!(!is_valid_hostname("+invalid"));
    assert!(is_valid_hostname("JellyDimension"));
    assert!(!is_valid_hostname("Jelly_Dimension"));
}

#[test]
fn test_username_validation() {
    assert!(is_acceptable_username("foo"));
    assert!(is_acceptable_username("cth451"));
    assert!(!is_acceptable_username("老白"));
    assert!(!is_acceptable_username("BAIMINGCONG"));
    assert!(!is_acceptable_username("root"));
    assert!(!is_acceptable_username("/root"));
    assert!(!is_acceptable_username("root:root"));
    assert!(!is_acceptable_username("root\n"));
    assert!(!is_acceptable_username("root\t"));
    assert!(!is_acceptable_username("ro ot"));
}

#[test]
fn test_path_strip() {
    for mount in BIND_MOUNTS {
        println!("{}", &mount[1..]);
    }
}

#[test]
fn test_set_fullname() {
    let passwd = r#"root:x:0:0:root:/root:/bin/bash
bin:x:1:1:bin:/dev/null:/bin/false
nobody:x:99:99:Unprivileged User:/dev/null:/bin/false
dbus:x:18:18:D-Bus Message Daemon User:/var/run/dbus:/bin/false
systemd-journal-gateway:x:994:994:systemd Journal Gateway:/:/sbin/nologin
systemd-bus-proxy:x:993:993:systemd Bus Proxy:/:/sbin/nologin
systemd-network:x:992:992:systemd Network Management:/:/sbin/nologin
systemd-resolve:x:991:991:systemd Resolver:/:/sbin/nologin
systemd-timesync:x:990:990:systemd Time Synchronization:/:/sbin/nologin
systemd-journal-remote:x:989:989:systemd Journal Remote:/:/sbin/nologin
systemd-journal-upload:x:988:988:systemd Journal Upload:/:/sbin/nologin
ldap:x:439:439:LDAP daemon owner:/var/lib/openldap:/bin/bash
http:x:207:207:HTTP daemon:/srv/http:/bin/true
uuidd:x:209:209:UUIDD user:/dev/null:/bin/true
locate:x:191:191:Locate daemon owner:/var/lib/mlocate:/bin/bash
polkitd:x:27:27:PolicyKit Daemon Owner:/etc/polkit-1:/bin/false
rtkit:x:133:133:RealtimeKit User:/proc:/sbin/nologin
named:x:40:40:BIND DNS Server:/var/named:/sbin/nologin
tss:x:159:159:Account used by the trousers package to sandbox the tcsd daemon:/dev/null:/sbin/nologin
unbound:x:986:986:unbound:/etc/unbound:/bin/false
systemd-coredump:x:985:985:systemd Core Dumper:/:/sbin/nologin
systemd-nobody:x:65534:65534:Unprivileged User (systemd):/dev/null:/bin/false
systemd-oom:x:980:980:systemd Userspace OOM Killer:/:/usr/bin/nologin
mysql:x:89:89:MariaDB Daemon User:/var/lib/mysql:/bin/false
dnsmasq:x:488:6:dnsmasq daemon owner:/:/bin/nologin
postgres:x:90:90:Postgres Daemon Owner:/var/lib/postgres:/bin/bash
avahi:x:84:84:Avahi Daemon Owner:/run/avahi-daemon:/bin/false
mongodb:x:300:6:MongoDB daemon owner:/var/lib/mongodb:/bin/bash
colord:x:124:124:Color Daemon Owner:/var/lib/colord:/bin/false
fcron:x:33:33::/var/spool/fcron:/bin/bash
flatpak:x:979:979:Flatpak system helper:/:/usr/bin/nologin
saned:x:978:978:SANE Daemon Owner:/:/usr/bin/nologin
sddm:x:977:977:Simple Desktop Display Manager Daemon Owner:/var/lib/sddm:/usr/bin/nologin
rpc:x:332:332:Rpcbind Daemon:/dev/null:/bin/false
usbmux:x:140:140:usbmux user:/:/sbin/nologin
nm-openconnect:x:104:104:NetworkManager user for OpenConnect:/:/sbin/nologin
saki:x:1000:1001::/home/saki:/bin/bash
pulse:x:58:58:PulseAudio Daemon Owner:/var/run/pulse:/bin/false
_apt:x:976:976::/var/lib/apt:/sbin/nologin
"#.split('\n').collect::<Vec<_>>();

    assert_eq!(set_full_name("Mag Mell", "saki", passwd.clone()).unwrap(), "root:x:0:0:root:/root:/bin/bash\nbin:x:1:1:bin:/dev/null:/bin/false\nnobody:x:99:99:Unprivileged User:/dev/null:/bin/false\ndbus:x:18:18:D-Bus Message Daemon User:/var/run/dbus:/bin/false\nsystemd-journal-gateway:x:994:994:systemd Journal Gateway:/:/sbin/nologin\nsystemd-bus-proxy:x:993:993:systemd Bus Proxy:/:/sbin/nologin\nsystemd-network:x:992:992:systemd Network Management:/:/sbin/nologin\nsystemd-resolve:x:991:991:systemd Resolver:/:/sbin/nologin\nsystemd-timesync:x:990:990:systemd Time Synchronization:/:/sbin/nologin\nsystemd-journal-remote:x:989:989:systemd Journal Remote:/:/sbin/nologin\nsystemd-journal-upload:x:988:988:systemd Journal Upload:/:/sbin/nologin\nldap:x:439:439:LDAP daemon owner:/var/lib/openldap:/bin/bash\nhttp:x:207:207:HTTP daemon:/srv/http:/bin/true\nuuidd:x:209:209:UUIDD user:/dev/null:/bin/true\nlocate:x:191:191:Locate daemon owner:/var/lib/mlocate:/bin/bash\npolkitd:x:27:27:PolicyKit Daemon Owner:/etc/polkit-1:/bin/false\nrtkit:x:133:133:RealtimeKit User:/proc:/sbin/nologin\nnamed:x:40:40:BIND DNS Server:/var/named:/sbin/nologin\ntss:x:159:159:Account used by the trousers package to sandbox the tcsd daemon:/dev/null:/sbin/nologin\nunbound:x:986:986:unbound:/etc/unbound:/bin/false\nsystemd-coredump:x:985:985:systemd Core Dumper:/:/sbin/nologin\nsystemd-nobody:x:65534:65534:Unprivileged User (systemd):/dev/null:/bin/false\nsystemd-oom:x:980:980:systemd Userspace OOM Killer:/:/usr/bin/nologin\nmysql:x:89:89:MariaDB Daemon User:/var/lib/mysql:/bin/false\ndnsmasq:x:488:6:dnsmasq daemon owner:/:/bin/nologin\npostgres:x:90:90:Postgres Daemon Owner:/var/lib/postgres:/bin/bash\navahi:x:84:84:Avahi Daemon Owner:/run/avahi-daemon:/bin/false\nmongodb:x:300:6:MongoDB daemon owner:/var/lib/mongodb:/bin/bash\ncolord:x:124:124:Color Daemon Owner:/var/lib/colord:/bin/false\nfcron:x:33:33::/var/spool/fcron:/bin/bash\nflatpak:x:979:979:Flatpak system helper:/:/usr/bin/nologin\nsaned:x:978:978:SANE Daemon Owner:/:/usr/bin/nologin\nsddm:x:977:977:Simple Desktop Display Manager Daemon Owner:/var/lib/sddm:/usr/bin/nologin\nrpc:x:332:332:Rpcbind Daemon:/dev/null:/bin/false\nusbmux:x:140:140:usbmux user:/:/sbin/nologin\nnm-openconnect:x:104:104:NetworkManager user for OpenConnect:/:/sbin/nologin\nsaki:x:1000:1001:Mag Mell:/home/saki:/bin/bash\npulse:x:58:58:PulseAudio Daemon Owner:/var/run/pulse:/bin/false\n_apt:x:976:976::/var/lib/apt:/sbin/nologin\n");
    assert!(set_full_name("Mag Mell\n", "saki", passwd.clone()).is_err());
    assert!(set_full_name("Mag Mell:", "saki", passwd.clone()).is_err());
}
