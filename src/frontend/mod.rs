use std::{
    convert::TryInto,
    io::{Read, Write},
    os::unix::prelude::AsRawFd,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Sender},
        Arc,
    },
    thread,
};

use crate::{disks, install, network};
use anyhow::{anyhow, Result};
use cursive::utils::{Counter, ProgressReader};
use nix::fcntl::FallocateFlags;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

mod cli;
mod games;
mod tui;

pub use cli::*;
use sha2::{Digest, Sha256};
pub use tui::tui_main;

pub(crate) enum InstallProgress {
    Pending(String, usize),
    Finished,
}

macro_rules! send_error {
    ($error_channel_tx_copy:ident, $e:ident) => {
        $error_channel_tx_copy.send($e.to_string()).unwrap();
        return;
    };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InstallConfig {
    variant: Option<Arc<network::VariantEntry>>,
    partition: Option<Arc<disks::Partition>>,
    mirror: Option<Arc<network::Mirror>>,
    user: Option<Arc<String>>,
    password: Option<Arc<String>>,
    hostname: Option<String>,
    locale: Option<Arc<String>>,
    timezone: Option<Arc<String>>,
    tc: Option<Arc<String>>,
    use_swap: Arc<AtomicBool>,
    swap_size: Arc<Option<f64>>,
    is_hibernation: Arc<AtomicBool>,
}

impl Default for InstallConfig {
    fn default() -> Self {
        InstallConfig {
            variant: None,
            partition: None,
            mirror: None,
            user: None,
            password: None,
            hostname: None,
            locale: None,
            timezone: None,
            tc: None,
            use_swap: Arc::new(AtomicBool::new(false)),
            swap_size: Arc::new(None),
            is_hibernation: Arc::new(AtomicBool::new(false)),
        }
    }
}

fn begin_install(
    sender: Sender<InstallProgress>,
    config: InstallConfig,
    tempdir: PathBuf,
) -> Result<()> {
    let refresh_interval = std::time::Duration::from_millis(30);
    let counter = Counter::new(0);
    let counter_clone = counter.clone();
    let url;
    let file_size: usize;
    let right_sha256;
    let extract_done: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let download_done: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let hasher_done: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    sender.send(InstallProgress::Pending(
        "Step 1 of 8: Formatting partitions ...".to_string(),
        0,
    ))?;

    let partition = &config.partition.unwrap();
    disks::format_partition(partition)?;
    let mount_path = install::auto_mount_root_path(&tempdir, partition)?;
    let mount_path_copy = mount_path.clone();
    let mut efi_path = mount_path.clone();
    if disks::is_efi_booted() {
        efi_path.push("efi");
        let mut esp_part = disks::find_esp_partition(partition.parent_path.as_ref().unwrap())?;
        std::fs::create_dir_all(&efi_path).unwrap();
        if esp_part.fs_type.is_none() {
            // format the un-formatted ESP partition
            esp_part.fs_type = Some("vfat".to_string());
            disks::format_partition(&esp_part)?;
        }
        install::mount_root_path(&esp_part, &efi_path)?;
    }
    if let Some(variant) = config.variant.as_ref() {
        let mirror_url = &config.mirror.as_ref().unwrap().url;
        file_size = variant.size.try_into().unwrap();
        url = format!("{}{}", mirror_url, variant.url);
        right_sha256 = variant.sha256sum.clone();
    } else {
        return Err(anyhow!(
            "Installer could not parse release metadata: `variant` field not found."
        ));
    }
    let extract_done_copy = extract_done.clone();
    let download_done_copy = download_done.clone();
    let hasher_done_copy = hasher_done.clone();
    let (sha256_work_tx, sha256_work_rx) = mpsc::channel();
    let (get_sha256_tx, get_sha256_rx) = mpsc::channel();
    let (error_channel_tx, error_channel_rx) = mpsc::channel();
    let error_channel_tx_copy = error_channel_tx.clone();
    let worker = thread::spawn(move || {
        let mut tarball_file = mount_path.clone();
        tarball_file.push("tarball");
        let mut output = match std::fs::File::create(tarball_file.clone()) {
            Ok(file) => file,
            Err(e) => {
                send_error!(error_channel_tx_copy, e);
            }
        };
        match network::download_file(url) {
            Ok(mut reader) => {
                if let Err(e) = nix::fcntl::fallocate(
                    output.as_raw_fd(),
                    FallocateFlags::empty(),
                    0,
                    file_size.try_into().unwrap(),
                ) {
                    let e = anyhow!(
                        "Installer failed to create temporary file for the download process:\n\n{}",
                        e
                    );
                    send_error!(error_channel_tx_copy, e);
                }
                if let Err(e) = output.flush() {
                    let e = anyhow!("Installer failed to save system release:\n\n{}\n\nPlease restart your installation environment.", e);
                    send_error!(error_channel_tx_copy, e);
                }
                let mut tarball_size = 0;
                loop {
                    let mut buf = vec![0; 4096];
                    let reader_size = match reader.read(&mut buf[..]) {
                        Ok(size) => size,
                        Err(e) => {
                            send_error!(error_channel_tx_copy, e);
                        }
                    };
                    tarball_size += reader_size;
                    if let Err(e) = output.write_all(&buf[..reader_size]) {
                        let e = anyhow!("Installer failed to write system release:\n\n{}", e);
                        send_error!(error_channel_tx_copy, e);
                    }
                    sha256_work_tx.send((buf, reader_size)).unwrap();
                    counter_clone.set(tarball_size);
                    if tarball_size == file_size {
                        download_done_copy.fetch_or(true, Ordering::SeqCst);
                        // dbg!("download complete");
                        break;
                    }
                }
                drop(sha256_work_tx);
                loop {
                    if hasher_done.load(Ordering::SeqCst) {
                        break;
                    }
                }
            }
            Err(e) => {
                let e = anyhow!("Installer failed to download system release:\n\n{}", e);
                send_error!(error_channel_tx_copy, e);
            }
        }
        counter_clone.set(0);
        match std::fs::File::open(tarball_file.clone()) {
            Ok(file) => output = file,
            Err(e) => {
                let e = anyhow!("Installer failed to read system release:\n\n{}", e);
                send_error!(error_channel_tx_copy, e);
            }
        }
        let reader = ProgressReader::new(counter_clone, output);
        if let Err(e) = install::extract_tar_xz(reader, &mount_path) {
            let e = anyhow!("Installer failed to unpack system release:\n\n{}", e);
            send_error!(error_channel_tx_copy, e);
        }
        extract_done_copy.fetch_or(true, Ordering::SeqCst);
        std::fs::remove_file(tarball_file).ok();
    });
    let sha256sum_work = thread::spawn(move || {
        let mut hasher = Sha256::new();
        loop {
            let rx = if let Ok(result) = sha256_work_rx.recv() {
                result
            } else {
                // dbg!("sha256sum complete");
                get_sha256_tx.send(hasher).unwrap();
                hasher_done_copy.fetch_or(true, Ordering::SeqCst);
                return;
            };
            let (buf, reader_size) = rx;
            if let Err(e) = hasher.write_all(&buf[..reader_size]) {
                let e = anyhow!(
                    "Installer failed to calculate checksum for system release:\n\n{}",
                    e
                );
                send_error!(error_channel_tx, e);
            }
        }
    });

    let file_size = file_size as f64;
    // Progress update
    loop {
        let tarball_downloaded_size = counter.get() as f64;
        let count = (tarball_downloaded_size / file_size * 100.0) as usize;
        if let Ok(err) = error_channel_rx.try_recv() {
            return Err(anyhow!(err));
        }
        sender.send(InstallProgress::Pending(
            "Step 2 of 8: Downloading system release ...".to_string(),
            count,
        ))?;
        std::thread::sleep(refresh_interval);
        if download_done.load(Ordering::SeqCst) {
            break;
        }
    }
    let mut fake_counter = 0;
    loop {
        sender.send(InstallProgress::Pending(
            "Step 3 of 8: Verifying system release ...".to_string(),
            fake_counter,
        ))?;
        std::thread::sleep(refresh_interval);
        if let Ok(hasher) = get_sha256_rx.try_recv() {
            let final_hash = hex::encode(hasher.finalize());
            if final_hash != right_sha256 {
                return Err(anyhow!(
                    "Installer detected a checksum mismatch in downloaded system release.\n\nExpected hash: {}\n\nCalculated hash: {}",
                    right_sha256,
                    final_hash
                ));
            }
            break;
        }
        fake_counter += 1;
        if fake_counter == 100 {
            fake_counter = 0;
        }
    }
    loop {
        let tarball_unpack_size = counter.get() as f64;
        let count = (tarball_unpack_size / file_size * 100.0) as usize;
        sender.send(InstallProgress::Pending(
            "Step 4 of 8: Unpacking system release ...".to_string(),
            count,
        ))?;
        std::thread::sleep(refresh_interval);
        if extract_done.load(Ordering::SeqCst) {
            break;
        }
    }
    // GC the worker thread
    worker.join().unwrap();
    sha256sum_work.join().unwrap();
    // genfstab to file
    install::genfstab_to_file(partition, &tempdir, Path::new("/"))?;
    if disks::is_efi_booted() {
        let esp_part = disks::find_esp_partition(partition.parent_path.as_ref().unwrap())?;
        install::genfstab_to_file(&esp_part, &tempdir, Path::new("/efi"))?;
    }
    let mut rng = thread_rng();
    let fake_counter: usize = rng.gen_range(0..100);
    sender.send(InstallProgress::Pending(
        "Step 5 of 8: Generating initramfs (initial RAM filesystem) ...".to_string(),
        fake_counter,
    ))?;
    let escape_vector = install::get_dir_fd(PathBuf::from("/"))?;
    install::dive_into_guest(&mount_path_copy)?;
    install::execute_dracut()?;
    let fake_counter: usize = rng.gen_range(0..100);
    sender.send(InstallProgress::Pending(
        "Step 6 of 8: Installing and configuring GRUB bootloader ...".to_string(),
        fake_counter,
    ))?;
    if disks::is_efi_booted() {
        install::execute_grub_install(None)?;
    } else {
        install::execute_grub_install(Some(partition.parent_path.as_ref().unwrap()))?;
    };
    let fake_counter: usize = rng.gen_range(0..100);
    sender.send(InstallProgress::Pending(
        "Step 7 of 8: Generating OpenSSH host keys ...".to_string(),
        fake_counter,
    ))?;
    install::gen_ssh_key()?;
    let fake_counter: usize = rng.gen_range(0..100);
    sender.send(InstallProgress::Pending(
        "Step 8 of 8: Finalising installation ...".to_string(),
        fake_counter,
    ))?;
    install::set_zoneinfo(&config.timezone.unwrap())?;
    install::set_hwclock_tc(match config.tc.unwrap().as_str() {
        "UTC" => true,
        "RTC" => false,
        _ => true,
    })?;
    install::set_hostname(&config.hostname.unwrap())?;
    let locale = config.locale.as_ref().unwrap();
    install::add_new_user(&config.user.unwrap(), &config.password.unwrap())?;
    install::execute_locale_gen(locale)?;
    install::set_locale(locale)?;
    let use_swap = config.use_swap.load(Ordering::SeqCst);
    if use_swap {
        if let Some(swap_size) = config.swap_size.as_ref() {
            install::create_swapfile(*swap_size, use_swap)?;
        }
    }
    // The swapfile offset reading problem is not solved yet, so hibernation is temporarily closed.
    install::disable_hibernate()?;
    install::escape_chroot(escape_vector.as_raw_fd())?;
    if disks::is_efi_booted() {
        install::umount_root_path(&efi_path)?;
    }
    install::remove_bind_mounts(&mount_path_copy)?;
    install::umount_root_path(&mount_path_copy).ok();
    sender.send(InstallProgress::Finished)?;

    Ok(())
}

#[cfg(all(not(feature = "is_retro"), target_arch = "x86_64"))]
#[test]
fn test_download_amd64() {
    use tempfile::TempDir;
    let json = r#"{"variant":{"name":"Base","size":821730832,"install_size":4157483520,"date":"20210602","sha256sum":"b5a5b9d889888a0e4f16b9f299b8a820ae2c8595aa363eb1e797d32ed0e957ed","url":"os-amd64/base/aosc-os_base_20210602_amd64.tar.xz"},"partition":{"path":"/dev/loop0p1","parent_path":"/dev/loop0","fs_type":"ext4","size":3145728},"mirror":{"name":"Beijing Foreign Studies University","name-tr":"bfsu-name","loc":"China","loc-tr":"bfsu-loc","url":"https://mirrors.bfsu.edu.cn/anthon/aosc-os/"},"user":"test","password":"test","hostname":"test","locale":"","continent":"Asia","city":"Shanghai","tc":"UTC"}"#;
    let config = serde_json::from_str(json).unwrap();
    let (tx, _rx) = std::sync::mpsc::channel();
    let tempdir = TempDir::new().unwrap().into_path();
    assert!(begin_install(tx, config, tempdir).is_ok());
}

#[test]
fn test_404() {
    use tempfile::TempDir;
    let json = r#"{"variant":{"name":"Base","size":821730832,"install_size":4157483520,"date":"20210602","sha256sum":"b5a5b9d889888a0e4f16b9f299b8a820ae2c8595aa363eb1e797d32ed0e957ed","url":"os-i486/base/aosc-os_base_20200620.1_i486.tar.xz"},"partition":{"path":"/dev/loop0p1","parent_path":"/dev/loop0","fs_type":"ext4","size":3145728},"mirror":{"name":"Beijing Foreign Studies University","name-tr":"bfsu-name","loc":"China","loc-tr":"bfsu-loc","url":"https://mirrors.bfsu.edu.cn/anthon/aosc-os/"},"user":"test","password":"test","hostname":"test","locale":"","continent":"Asia","city":"Shanghai","tc":"UTC","use_swap":false,"swap_size":null,"is_hibernation":false}"#;
    let config = serde_json::from_str(json).unwrap();
    let (tx, _rx) = std::sync::mpsc::channel();
    let tempdir = TempDir::new().unwrap().into_path();
    assert!(begin_install(tx, config, tempdir).is_err());
}

#[cfg(all(feature = "is_retro", target_arch = "x86"))]
#[test]
fn test_download_i486() {
    use tempfile::TempDir;
    let json = r#"{"variant":{"name":"Base","size":97613332,"install_size":448060928,"date":"20220128","sha256sum":"2b691be7f14c4948fac7e1533bd5a19e78ee72640f666b64f2c1fae2216ab708","url":"os-i486/base/aosc-os_base_20220128_i486.tar.xz"},"partition":{"path":"/dev/loop0p1","parent_path":"/dev/loop0","fs_type":"ext4","size":3145728},"mirror":{"name":"Beijing Foreign Studies University","name-tr":"bfsu-name","loc":"China","loc-tr":"bfsu-loc","url":"https://mirrors.bfsu.edu.cn/anthon/aosc-os/"},"user":"test","password":"test","hostname":"test","locale":"C.UTF-8","continent":"Asia","city":"Shanghai","tc":"UTC","use_swap":false,"swap_size":null,"is_hibernation":false}}"#;
    let config = serde_json::from_str(json).unwrap();
    let (tx, _rx) = std::sync::mpsc::channel();
    let tempdir = TempDir::new().unwrap().into_path();
    let (_tx2, rx2) = std::sync::mpsc::channel();
    assert!(begin_install(tx, config, tempdir, rx2).is_ok());
}
