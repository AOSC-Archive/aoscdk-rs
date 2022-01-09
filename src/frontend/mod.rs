use std::{
    convert::TryInto,
    io::{Read, Write},
    os::unix::prelude::AsRawFd,
    path::PathBuf,
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

mod tui;

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
    continent: Option<Arc<String>>,
    city: Option<Arc<String>>,
    tc: Option<Arc<String>>,
}

fn begin_install(sender: Sender<InstallProgress>, config: InstallConfig) -> Result<()> {
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
        "Step 1 of 6: Formatting partitions ...".to_string(),
        0,
    ))?;

    let partition = &config.partition.unwrap();
    disks::format_partition(partition)?;
    let mount_path = install::auto_mount_root_path(partition)?;
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
        return Err(anyhow!("Internal error: no variant field found."));
    }
    let extract_done_copy = extract_done.clone();
    let download_done_copy = download_done.clone();
    let hasher_done_copy = hasher_done.clone();
    let (error_channel_tx, error_channel_rx) = mpsc::channel();
    let (sha256_work_tx, sha256_work_rx) = mpsc::channel();
    let (get_sha256_tx, get_sha256_rx) = mpsc::channel();
    let error_channel_tx_copy = error_channel_tx.clone();
    let error_channel_tx_copy_2 = error_channel_tx;
    let worker = thread::spawn(move || {
        let mut tarball_file = mount_path.clone();
        tarball_file.push("tarball");
        let mut output;
        match std::fs::File::create(tarball_file.clone()) {
            Ok(file) => output = file,
            Err(e) => {
                send_error!(error_channel_tx_copy, e);
            }
        }
        if let Ok(mut reader) = network::download_file(&url) {
            if let Err(e) = nix::fcntl::fallocate(
                output.as_raw_fd(),
                FallocateFlags::empty(),
                0,
                file_size.try_into().unwrap(),
            ) {
                let e = anyhow!("Failed to create a file using fallocate! {}", e);
                send_error!(error_channel_tx_copy, e);
            }
            if let Err(e) = output.flush() {
                let e = anyhow!("Failed to fallocate flush! {}", e);
                send_error!(error_channel_tx_copy, e);
            }
            let mut tarball_size = 0;
            loop {
                let mut buf = vec![0; 4096];
                let reader_size;
                match reader.read(&mut buf[..]) {
                    Ok(size) => reader_size = size,
                    Err(e) => {
                        send_error!(error_channel_tx_copy, e);
                    }
                };
                tarball_size += reader_size;
                if let Err(e) = output.write_all(&buf[..reader_size]) {
                    let e = anyhow!("Failed to write file! {}", e);
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
        } else {
            return;
        }
        counter_clone.set(0);
        match std::fs::File::open(tarball_file.clone()) {
            Ok(file) => output = file,
            Err(e) => {
                let e = anyhow!("Failed to open tarball! {}", e);
                send_error!(error_channel_tx_copy, e);
            }
        }
        let reader = ProgressReader::new(counter_clone, output);
        if let Err(e) = install::extract_tar_xz(reader, &mount_path) {
            let e = anyhow!("Failed to extract tarball! {}", e);
            send_error!(error_channel_tx_copy, e);
        }
        extract_done_copy.fetch_or(true, Ordering::SeqCst);
        std::fs::remove_file(tarball_file).ok();
    });
    let sha256sum_work = thread::spawn(move || {
        let mut hasher = Sha256::new();
        loop {
            let rx;
            if let Ok(result) = sha256_work_rx.recv() {
                rx = result;
            } else {
                // dbg!("sha256sum complete");
                get_sha256_tx.send(hasher).unwrap();
                hasher_done_copy.fetch_or(true, Ordering::SeqCst);
                return;
            }
            let (buf, reader_size) = rx;
            if let Err(e) = hasher.write_all(&buf[..reader_size]) {
                let e = anyhow!("Failed to write hasher! {}", e);
                send_error!(error_channel_tx_copy_2, e);
            }
        }
    });

    // Progress update
    loop {
        sender.send(InstallProgress::Pending(
            "Step 2 of 6: Downloading system release ...".to_string(),
            counter.get() * 100 / file_size,
        ))?;
        std::thread::sleep(refresh_interval);
        if let Ok(err) = error_channel_rx.try_recv() {
            return Err(anyhow!(err));
        }
        if download_done.load(Ordering::SeqCst) {
            break;
        }
    }
    let mut fake_counter = 0;
    loop {
        sender.send(InstallProgress::Pending(
            "Step 3 of 6: Verifying system release ...".to_string(),
            fake_counter,
        ))?;
        std::thread::sleep(refresh_interval);
        if let Ok(hasher) = get_sha256_rx.try_recv() {
            let final_hash = hex::encode(hasher.finalize());
            if final_hash != right_sha256 {
                return Err(anyhow!(
                    "Network error: checksum do not match! \nright hash: {}\nfinal hash: {}",
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
        sender.send(InstallProgress::Pending(
            "Step 4 of 6: Unpacking system release ...".to_string(),
            counter.get() * 100 / file_size,
        ))?;
        std::thread::sleep(refresh_interval);
        if extract_done.load(Ordering::SeqCst) {
            break;
        }
    }
    // GC the worker thread
    worker.join().unwrap();
    sha256sum_work.join().unwrap();
    let mut rng = thread_rng();
    let fake_counter: usize = rng.gen_range(0..100);
    sender.send(InstallProgress::Pending(
        "Step 5 of 6: Generating initramfs (initial RAM filesystem) ...".to_string(),
        fake_counter,
    ))?;
    let escape_vector = install::get_dir_fd(PathBuf::from("/"))?;
    install::dive_into_guest(&mount_path_copy)?;
    install::execute_dracut()?;
    let fake_counter: usize = rng.gen_range(0..100);
    sender.send(InstallProgress::Pending(
        "Step 6 of 6: Installing and configuring GRUB bootloader ...".to_string(),
        fake_counter,
    ))?;

    if disks::is_efi_booted() {
        install::execute_grub_install(None)?;
    } else {
        install::execute_grub_install(Some(partition.parent_path.as_ref().unwrap()))?;
    };
    install::set_zoneinfo(
        format!("{}/{}", &config.continent.unwrap(), &config.city.unwrap()).as_str(),
    )?;
    install::set_hwclock_tc(match config.tc.unwrap().as_str() {
        "UTC" => true,
        "RTC" => false,
        _ => true,
    })?;
    install::set_hostname(&config.hostname.unwrap())?;
    let locale = config.locale.as_ref().unwrap();
    install::add_new_user(&config.user.unwrap(), &config.password.unwrap())?;
    install::execute_locale_gen(&locale)?;
    install::set_locale(&locale)?;
    install::escape_chroot(escape_vector)?;
    if disks::is_efi_booted() {
        install::umount_root_path(&efi_path)?;
    }
    install::remove_bind_mounts(&mount_path_copy)?;
    install::umount_root_path(&mount_path_copy).ok();
    sender.send(InstallProgress::Finished)?;

    Ok(())
}

#[test]
fn test_download() {
    let json = r#"{"variant":{"name":"Base","size":821730832,"install_size":4157483520,"date":"20210602","sha256sum":"b5a5b9d889888a0e4f16b9f299b8a820ae2c8595aa363eb1e797d32ed0e957ed","url":"os-amd64/base/aosc-os_base_20210602_amd64.tar.xz"},"partition":{"path":"/dev/loop0p1","parent_path":"/dev/loop0","fs_type":"ext4","size":3145728},"mirror":{"name":"Beijing Foreign Studies University","name-tr":"bfsu-name","loc":"China","loc-tr":"bfsu-loc","url":"https://mirrors.bfsu.edu.cn/anthon/aosc-os/"},"user":"test","password":"test","hostname":"test","locale":"","continent":"Asia","city":"Shanghai","tc":"UTC"}"#;
    let config = serde_json::from_str(json).unwrap();
    let (tx, _rx) = std::sync::mpsc::channel();
    begin_install(tx, config).unwrap();
}
