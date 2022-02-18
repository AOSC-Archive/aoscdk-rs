use anyhow::{anyhow, Result};
use nix::dir::Dir;
use nix::fcntl::OFlag;
use nix::mount;
use nix::sys::reboot::{reboot, RebootMode};
use nix::sys::stat::Mode;
use nix::unistd::{chroot, fchdir, sync};
use std::io::prelude::*;
use std::os::unix::io::AsRawFd;
use std::os::unix::prelude::OsStrExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{fs::File, path::Path};

use crate::disks::{fstab_entries, is_efi_booted, Partition};
use crate::parser::{list_zoneinfo, locale_names};

const BIND_MOUNTS: &[&str] = &["/dev", "/proc", "/sys", "/run/udev"];
const BUNDLED_LOCALE_GEN: &[u8] = include_bytes!("../res/locale.gen");
const SYSTEM_LOCALE_GEN_PATH: &str = "/etc/locale.gen";
const SYSTEM_ZONEINFO1970_PATH: &str = "/usr/share/zoneinfo/zone1970.tab";
const BUNDLED_ZONEINFO_LIST: &[u8] = include_bytes!("../res/zone1970.tab");

fn read_system_locale_list() -> Result<Vec<u8>> {
    let mut f = std::fs::File::open(SYSTEM_LOCALE_GEN_PATH)?;
    let mut data: Vec<u8> = Vec::new();
    data.reserve(8800);
    f.read_to_end(&mut data)?;

    Ok(data)
}

/// Get the list of available locales
pub fn get_locale_list() -> Result<Vec<String>> {
    let data = if !cfg!(feature = "is_retro") {
        read_system_locale_list().unwrap_or_else(|_| BUNDLED_LOCALE_GEN.to_vec())
    } else {
        BUNDLED_LOCALE_GEN.to_vec()
    };
    let names = locale_names(&data).map_err(|_| anyhow!("Could not parse system locale list"))?;
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

pub fn get_zoneinfo_list() -> Result<Vec<(String, Vec<String>)>> {
    let data = read_system_zoneinfo_list().unwrap_or_else(|_| BUNDLED_ZONEINFO_LIST.to_vec());
    let mut zoneinfo_list = list_zoneinfo(&data)
        .map_err(|_| anyhow!("Could not parse zoneinfo list"))?
        .1;
    if zoneinfo_list.is_empty() {
        return Err(anyhow!("zoneinfo list is empty!"));
    }
    zoneinfo_list.sort();
    let mut continent = zoneinfo_list[0].split_once("/").unwrap().0;
    let mut result = Vec::new();
    let mut city = Vec::new();
    for i in &zoneinfo_list {
        let split_name = i.split_once("/").unwrap();
        if split_name.0 == continent {
            city.push(split_name.1.to_string());
        } else {
            result.push((continent.to_string(), city.clone()));
            city.clear();
            city.push(split_name.1.to_string());
            continent = split_name.0;
        }
    }

    Ok(result)
}

/// Extract the given .tar.xz stream and preserve all the file attributes
pub fn extract_tar_xz<R: Read>(reader: R, path: &Path) -> Result<()> {
    let decompress = xz2::read::XzDecoder::new(reader);
    let mut tar_processor = tar::Archive::new(decompress);
    tar_processor.set_unpack_xattrs(true);
    tar_processor.set_preserve_permissions(true);
    tar_processor.set_preserve_ownerships(true);
    tar_processor.unpack(path)?;

    Ok(())
}

/// Mount the filesystem to a temporary directory
pub fn auto_mount_root_path(tmp_path: &Path, partition: &Partition) -> Result<PathBuf> {
    mount_root_path(partition, tmp_path)?;

    Ok(tmp_path.to_path_buf())
}

/// Sync the filesystem and then reboot IMMEDIATELY (ignores init)
pub fn sync_and_reboot() -> Result<()> {
    sync();
    reboot(RebootMode::RB_AUTOBOOT)?;

    Ok(())
}

/// Mount the filesystem
pub fn mount_root_path(partition: &Partition, target: &Path) -> Result<()> {
    if partition.fs_type.is_none() || partition.path.is_none() {
        return Err(anyhow!("Path not specified."));
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
    let s = fstab_entries(partition, mount_path)?;
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
pub fn escape_chroot(root_fd: Dir) -> Result<()> {
    fchdir(root_fd.as_raw_fd())?;
    chroot(".")?;
    std::env::set_current_dir("/")?; // reset cwd (on host)

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
    let output = Command::new("/usr/bin/update-initramfs").output()?;
    if !output.status.success() {
        return Err(anyhow!(
            "Failed to execute dracut: \n{}\n{}",
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        ));
    }

    Ok(())
}

/// Runs locale-gen
/// Must be used in a chroot context
#[cfg(feature = "is_retro")]
pub fn execute_locale_gen(locale: &str) -> Result<()> {
    use std::io::SeekFrom;
    let mut locale_gen_file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open("/etc/locale.gen")?;
    let mut buf = String::new();
    locale_gen_file.read_to_string(&mut buf)?;
    let mut locale_gen_list = buf.split("\n").map(|x| x.into()).collect::<Vec<String>>();
    let index = locale_gen_list
        .iter()
        .position(|x| x.starts_with(&format!("#{}", locale)))
        .ok_or_else(|| anyhow!("Cannot get this locale from list!"))?;
    let selected_locale = &locale_gen_list[index];
    let split_selected_locale = selected_locale.split_once(".");
    if split_selected_locale.is_some() {
        locale_gen_list[index] = locale_gen_list[index].replace("#", "");
    } else {
        let match_index_list = locale_gen_list
            .iter()
            .enumerate()
            .filter(|(_, x)| x.starts_with(&format!("#{}", locale)))
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        for i in match_index_list {
            locale_gen_list[i] = locale_gen_list[i].replace("#", "");
        }
    }
    let locale_gen_str = locale_gen_list.join("\n");
    locale_gen_file.seek(SeekFrom::Start(0))?;
    locale_gen_file.write_all(locale_gen_str.as_bytes())?;
    let output = Command::new("/usr/bin/locale-gen").output()?;
    if !output.status.success() {
        return Err(anyhow!(
            "Failed to execute locale-gen: \n{}\n{}",
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        ));
    }

    Ok(())
}

/// Runs locale-gen (dummy function for non-retro mode)
/// Must be used in a chroot context
#[cfg(not(feature = "is_retro"))]
pub fn execute_locale_gen(_locale: &str) -> Result<()> {
    Ok(())
}

/// Runs dracut (dummy function for retro mode)
/// Must be used in a chroot context
#[cfg(feature = "is_retro")]
pub fn execute_dracut() -> Result<()> {
    Ok(())
}

/// Runs ssh-keygen -A (dummy function for non-retro mode)
/// Must be used in a chroot context
#[cfg(not(feature = "is_retro"))]
pub fn gen_ssh_key() -> Result<()> {
    Ok(())
}

/// Runs ssh-keygen -A
/// Must be used in a chroot context
#[cfg(feature = "is_retro")]
pub fn gen_ssh_key() -> Result<()> {
    let output = Command::new("ssh-keygen").arg("-A").output()?;
    if !output.status.success() {
        return Err(anyhow!(
            "Failed to execute ssh-keygen: \n{}\n{}",
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        ));
    }

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
    std::os::unix::fs::symlink(format!("/usr/share/zoneinfo/{}", zone), "/etc/localtime")?;

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
        if line.len() < 3 || line[2] == "UTC" {
            false
        } else {
            line[2] == "LOCAL"
        }
    } else {
        false
    };
    if utc {
        if !status_is_rtc {
            return Ok(());
        } else {
            let command = Command::new("hwclock").arg("-wu").output()?;
            if !command.status.success() {
                return Err(anyhow!(
                    "Failed to set UTC: {}",
                    String::from_utf8_lossy(&command.stderr)
                ));
            }
        }
    } else if status_is_rtc {
        return Ok(());
    } else {
        let command = Command::new("hwclock").arg("-wl").output()?;
        if !command.status.success() {
            return Err(anyhow!(
                "Failed to set RTC: {}",
                String::from_utf8_lossy(&command.stderr)
            ));
        }
    }

    Ok(())
}

/// Adds a new normal user to the guest environment
/// Must be used in a chroot context
pub fn add_new_user(name: &str, password: &str) -> Result<()> {
    let command = Command::new("useradd")
        .args(&["-m", "-s", "/bin/bash", name])
        .output()?;
    if !command.status.success() {
        return Err(anyhow!(
            "Failed to add a new user: {}",
            String::from_utf8_lossy(&command.stderr)
        ));
    }
    let command = Command::new("usermod")
        .args(&["-aG", "audio,cdrom,video,wheel", name])
        .output()?;
    if !command.status.success() {
        return Err(anyhow!(
            "Failed to add a new user: {}",
            String::from_utf8_lossy(&command.stderr)
        ));
    }
    let command = Command::new("chpasswd").stdin(Stdio::piped()).spawn()?;
    let mut stdin = command.stdin.unwrap();
    stdin.write_all(format!("{}:{}\n", name, password).as_bytes())?;
    stdin.flush()?;

    Ok(())
}

/// Runs grub-install and grub-mkconfig
/// Must be used in a chroot context
pub fn execute_grub_install(mbr_dev: Option<&PathBuf>) -> Result<()> {
    let mut command = Command::new("grub-install");
    let cmd;
    if let Some(mbr_dev) = mbr_dev {
        cmd = command.arg("--target=i386-pc").arg(mbr_dev);
    } else {
        cmd = command
            .arg("--target=x86_64-efi")
            .arg("--bootloader-id=AOSC OS")
            .arg("--efi-directory=/efi");
    }
    let process = cmd.output()?;
    if !process.status.success() {
        return Err(anyhow!(
            "Failed to execute grub-install: {}",
            String::from_utf8_lossy(&process.stderr)
        ));
    }
    let process = Command::new("grub-mkconfig")
        .arg("-o")
        .arg("/boot/grub/grub.cfg")
        .output()?;
    if !process.status.success() {
        return Err(anyhow!(
            "Failed to execute grub-mkconfig: {}",
            String::from_utf8_lossy(&process.stderr)
        ));
    }

    Ok(())
}

/// Run umount -R
/// Test in Livekit only
pub fn umount_all(mount_path: &Path, root_fd: Dir) {
    escape_chroot(root_fd).ok();
    let efi_path = mount_path.join("efi");
    if is_efi_booted() {
        umount_root_path(&efi_path).ok();
    }
    umount_root_path(mount_path).ok();
}

#[test]
fn test_path_strip() {
    for mount in BIND_MOUNTS {
        println!("{}", &mount[1..]);
    }
}
