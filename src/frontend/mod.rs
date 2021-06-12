use std::{
    convert::TryInto,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::Sender,
        Arc,
    },
    thread,
};

use crate::{disks, install, network};
use anyhow::{anyhow, Result};
use cursive::utils::{Counter, ProgressReader};

mod tui;

pub use tui::tui_main;

pub(crate) enum InstallProgress {
    Pending(String, usize),
    Finished,
}

#[derive(Debug, Clone)]
struct InstallConfig {
    variant: Option<Arc<network::VariantEntry>>,
    partition: Option<Arc<disks::Partition>>,
    mirror: Option<Arc<network::Mirror>>,
    user: Option<Arc<String>>,
    password: Option<Arc<String>>,
    hostname: Option<String>,
    locale: Option<Arc<String>>,
}

fn begin_install(sender: Sender<InstallProgress>, config: InstallConfig) -> Result<()> {
    let refresh_interval = std::time::Duration::from_millis(30);
    let counter = Counter::new(0);
    let counter_clone = counter.clone();
    let url;
    let file_size: usize;
    let download_done: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let extract_done: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    sender.send(InstallProgress::Pending(
        "Step 1 of 5: Formatting partitions ...".to_string(),
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
    } else {
        return Err(anyhow!("Internal error: no variant field found."));
    }
    let download_done_copy = download_done.clone();
    let extract_done_copy = extract_done.clone();
    let worker = thread::spawn(move || {
        let mut tarball_file = mount_path.clone();
        tarball_file.push("tarball");
        let mut output;
        if let Ok(reader) = network::download_file(&url) {
            let mut reader = ProgressReader::new(counter_clone.clone(), reader);
            output = std::fs::File::create(tarball_file.clone()).unwrap();
            std::io::copy(&mut reader, &mut output).unwrap();
            download_done_copy.fetch_or(true, Ordering::SeqCst);
        } else {
            return;
        }
        counter_clone.set(0);
        output = std::fs::File::open(tarball_file.clone()).unwrap();
        let reader = ProgressReader::new(counter_clone, output);
        install::extract_tar_xz(reader, &mount_path).unwrap();
        extract_done_copy.fetch_or(true, Ordering::SeqCst);
        std::fs::remove_file(tarball_file).ok();
    });

    // Progress update
    loop {
        sender.send(InstallProgress::Pending(
            "Step 2 of 5: Downloading system release ...".to_string(),
            counter.get() * 100 / file_size,
        ))?;
        std::thread::sleep(refresh_interval);
        if download_done.load(Ordering::SeqCst) {
            break;
        }
    }
    loop {
        sender.send(InstallProgress::Pending(
            "Step 3 of 5: Extracting system release ...".to_string(),
            counter.get() * 100 / file_size,
        ))?;
        std::thread::sleep(refresh_interval);
        if extract_done.load(Ordering::SeqCst) {
            break;
        }
    }
    // GC the worker thread
    worker.join().unwrap();
    sender.send(InstallProgress::Pending(
        "Step 4 of 5: Generating initramfs (initial RAM filesystem) ...".to_string(),
        0,
    ))?;

    let escape_vector = install::get_dir_fd(PathBuf::from("/"))?;
    install::dive_into_guest(&mount_path_copy)?;
    install::execute_dracut()?;
    sender.send(InstallProgress::Pending(
        "Step 5 of 5: Installing and configuring GRUB bootloader ...".to_string(),
        0,
    ))?;

    if disks::is_efi_booted() {
        install::execute_grub_install(None)?;
    } else {
        install::execute_grub_install(Some(partition.parent_path.as_ref().unwrap()))?;
    };
    install::set_hostname(&config.hostname.unwrap())?;
    install::add_new_user(&config.user.unwrap(), &config.password.unwrap())?;
    install::set_locale(&config.locale.as_ref().unwrap())?;
    install::escape_chroot(escape_vector)?;
    if disks::is_efi_booted() {
        install::umount_root_path(&efi_path)?;
    }
    install::remove_bind_mounts(&mount_path_copy)?;
    install::umount_root_path(&mount_path_copy).ok();
    sender.send(InstallProgress::Finished)?;

    Ok(())
}
