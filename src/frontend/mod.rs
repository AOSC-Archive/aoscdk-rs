use std::{
    convert::TryInto,
    io::Write,
    os::unix::prelude::AsRawFd,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Sender},
        Arc,
    },
    thread,
};

use crate::{
    disks,
    install::{self, log_system_info},
    network, DEPLOYKIT_USER_AGENT,
};
use anyhow::{anyhow, Result};
use cursive::utils::Counter;
use log::info;
use nix::fcntl::FallocateFlags;
use rand::{thread_rng, Rng};
use serde::{de::Visitor, Deserialize, Serialize};
use std::sync::atomic;

mod cli;
mod games;
mod tui;

pub use cli::*;
use sha2::{Digest, Sha256};
pub use tui::tui_main;

pub const DEFAULT_EMPTY_SIZE: u64 = 5 * 1024 * 1024 * 1024;

const STEP1: &str = "Step 1 of 8: Formatting partitions";
const STEP2: &str = "Step 2 of 8: Downloading system release";
const STEP3: &str = "Step 3 of 8: Verifying system release";
const STEP4: &str = "Step 4 of 8: Unpacking system release ...";
const STEP5: &str = "Step 5 of 8: Generating initramfs (initial RAM filesystem)";
const STEP6: &str = "Step 6 of 8: Installing and configuring GRUB bootloader";
const STEP7: &str = "Step 7 of 8: Generating OpenSSH host keys";
const STEP8: &str = "Step 8 of 8: Finalising installation";

pub(crate) enum InstallProgress {
    Pending(String, usize, Option<(String, String)>),
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
    full_name: Option<Arc<String>>,
    user: Option<Arc<String>>,
    password: Option<Arc<String>>,
    hostname: Option<String>,
    locale: Option<Arc<String>>,
    timezone: Option<Arc<String>>,
    tc: Option<Arc<String>>,
    use_swap: Arc<AtomicBoolWrapper>,
    swap_size: Arc<Option<f64>>,
    is_hibernation: Arc<AtomicBoolWrapper>,
}

impl Default for InstallConfig {
    fn default() -> Self {
        InstallConfig {
            variant: None,
            partition: None,
            mirror: None,
            full_name: None,
            user: None,
            password: None,
            hostname: None,
            locale: None,
            timezone: None,
            tc: None,
            use_swap: Arc::new(AtomicBoolWrapper {
                v: AtomicBool::new(false),
            }),
            swap_size: Arc::new(None),
            is_hibernation: Arc::new(AtomicBoolWrapper {
                v: AtomicBool::new(false),
            }),
        }
    }
}

#[derive(Debug)]
pub struct AtomicBoolWrapper {
    v: AtomicBool,
}

struct AtomicBoolWrapperVisitor;

impl AtomicBoolWrapperVisitor {
    fn new() -> Self {
        Self {}
    }
}

impl<'de> Visitor<'de> for AtomicBoolWrapperVisitor {
    type Value = AtomicBoolWrapper;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "value is not a bool")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(AtomicBoolWrapper {
            v: AtomicBool::new(v),
        })
    }
}

impl Serialize for AtomicBoolWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bool(self.v.load(atomic::Ordering::SeqCst))
    }
}

impl<'de> Deserialize<'de> for AtomicBoolWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_bool(AtomicBoolWrapperVisitor::new())
    }
}

fn begin_install(
    sender: Sender<InstallProgress>,
    config: InstallConfig,
    tempdir: PathBuf,
    logfile: PathBuf,
) -> Result<()> {
    log_system_info();

    info!("Prepare trying unmount before deploykit mount partition ...");
    install::prepare_try_umount()?;

    let refresh_interval = std::time::Duration::from_millis(30);
    let counter = Counter::new(0);
    let url;
    let file_size: usize;
    let right_sha256;
    let extract_done: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let download_done: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
    let hasher_done: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

    sender.send(InstallProgress::Pending(STEP1.to_string(), 0, None))?;
    info!("{}", STEP1);

    let partition = &config.partition.unwrap();

    info!("Formatting partitions: {:?}", partition);
    disks::format_partition(partition)?;

    info!("Mounting partitions: {:?}", partition);
    let mount_path = install::auto_mount_root_path(&tempdir, partition)?;
    let mount_path_copy = mount_path.clone();
    let mut efi_path = mount_path.clone();
    if disks::is_efi_booted() {
        efi_path.push("efi");

        info!("Finding ESP partition from: {:?}", partition.parent_path);
        let mut esp_part = disks::find_esp_partition(partition.parent_path.as_ref().unwrap())?;
        info!("ESP is: {:?}", esp_part);

        std::fs::create_dir_all(&efi_path).unwrap();
        if esp_part.fs_type.is_none() {
            // format the un-formatted ESP partition
            esp_part.fs_type = Some("vfat".to_string());

            info!("Formatting ESP partition: {:?}", esp_part);
            disks::format_partition(&esp_part)?;
        }
        install::mount_root_path(&esp_part, &efi_path)?;
    }
    if let Some(variant) = config.variant.as_ref() {
        let mirror_url = &config.mirror.as_ref().unwrap().url;
        file_size = variant.size.try_into().unwrap();
        url = format!("{}{}", mirror_url, variant.url);
        right_sha256 = variant.sha256sum.clone();

        info!(
            "Mirror URL is: {}, file_size: {}, url: {}, right_sha256: {}",
            mirror_url, file_size, url, right_sha256
        );
    } else {
        return Err(anyhow!(
            "Installer could not parse release metadata: `variant` field not found."
        ));
    }

    let use_swap = config.use_swap.v.load(Ordering::SeqCst);
    if use_swap {
        if let Some(swap_size) = config.swap_size.as_ref() {
            info!("Creating swapfile and trying swapon swapfile ...");
            install::create_swapfile(*swap_size, use_swap, &tempdir)?;
        }
    }

    let extract_done_copy = extract_done.clone();
    let download_done_copy = download_done.clone();
    let hasher_done_copy = hasher_done.clone();
    let (sha256_work_tx, sha256_work_rx) = mpsc::channel();
    let (get_sha256_tx, get_sha256_rx) = mpsc::channel();
    let (error_channel_tx, error_channel_rx) = mpsc::channel();
    let error_channel_tx_copy = error_channel_tx.clone();

    let (vaild_tx, vaild_rx) = std::sync::mpsc::channel();

    let cc = counter.clone();

    let worker = thread::spawn(move || {
        let mut tarball_file = mount_path.clone();
        tarball_file.push("tarball");
        let mut output = match std::fs::File::create(tarball_file.clone()) {
            Ok(file) => {
                info!("tarball file: {:?} is created", tarball_file);

                file
            }
            Err(e) => {
                send_error!(error_channel_tx_copy, e);
            }
        };

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(2)
            .build()
            .unwrap();
        let client = reqwest::Client::builder()
            .user_agent(DEPLOYKIT_USER_AGENT!())
            // .timeout(Duration::from_secs(10))
            .build()
            .unwrap();

        let urlc = url.clone();

        let tbl_file_c = tarball_file.clone();

        let ccc = cc.clone();

        let error_channel_tx_copy_copy = error_channel_tx_copy.clone();

        // let (speed_tx, speed_rx) = std::sync::mpsc::channel();

        runtime.block_on(async move {
            let mut resp = match client.get(urlc).send().await {
                Ok(resp) => resp,
                Err(e) => {
                    send_error!(error_channel_tx_copy, e);
                }
            };

            info!("Allocating tarball file: {:?}", &tbl_file_c);
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

            info!("Flushing tarball_file: {:?}", &tbl_file_c);
            if let Err(e) = output.flush() {
                let e = anyhow!("Installer failed to save system release:\n\n{}\n\nPlease restart your installation environment.", e);
                send_error!(error_channel_tx_copy, e);
            }
            
            let mut tarball_size = 0;

            let mut timer = tokio::time::Instant::now();
            let mut tarball_size_1s = 0;

            loop {
                if tarball_size == file_size {
                    info!("Download complete");
                    download_done_copy.fetch_or(true, Ordering::SeqCst);
                    break;
                }
                match resp.chunk().await {
                    Ok(v) => {
                        if let Some(chunk) = v {
                            let now = timer.elapsed().as_secs_f64();
                            if now >= 1.0 {
                                let speed = tarball_size_1s as f64 / 1024.0 / now;
                                let eta = (file_size - tarball_size) as f64 / 1024.0 / speed;
                                let s = if speed > 1000.0 * 1000.0 {
                                    format!("{:.1}GiB/s", speed / 1024.0 / 1024.0)
                                } else if speed > 1000.0 {
                                    format!("{:.1}MiB/s", speed / 1024.0)
                                } else {
                                    format!("{:.1}KiB/s", speed)
                                };

                                let s2 = if eta >= 60.0 * 60.0 * 24.0 {
                                    format!("{:.1}d", (eta / 60.0 / 60.0 / 24.0).round())
                                } else if eta >= 60.0 * 60.0 {
                                    format!("{:.1}h", (eta / 60.0 / 60.0).round())
                                } else if eta >= 60.0 {
                                    format!("{:.1}m", (eta / 60.0).round())
                                } else {
                                    format!("{:.0}s", eta.round())
                                };
                                vaild_tx.send((s, s2)).unwrap();
                                tarball_size_1s = 0;
                                timer = tokio::time::Instant::now();
                            } else {
                                tarball_size_1s += chunk.len();
                            }
                            
                            if let Err(e) = output.write_all(&chunk) {
                                send_error!(error_channel_tx_copy, e);
                            }
                            tarball_size += chunk.len();
                            cc.set(tarball_size);
                            sha256_work_tx.send((chunk.to_vec(), chunk.len())).unwrap();
                        } else {
                            let e = "chunk is none".to_string();
                            send_error!(error_channel_tx_copy, e);
                        }
                    }
                    Err(e) => {
                         send_error!(error_channel_tx_copy, e);
                    }
                }
            }
        });

        info!("Trying extract tarball file: {:?}", &tarball_file);

        ccc.set(0);

        // let cc = cc.clone();

        if let Err(e) = install::extract_file(
            file_size as f64,
            url,
            &tarball_file,
            &mount_path,
            ccc,
        ) {
            let e = anyhow!("Installer failed to unpack system release:\n\n{}", e);
            send_error!(error_channel_tx_copy_copy, e);
        }

        extract_done_copy.fetch_or(true, Ordering::SeqCst);

        info!("Trying remove tarball file: {:?}", tarball_file);
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
    info!("{}", STEP2);
    loop {
        // let counter_clone = counter.clone();
        let tarball_downloaded_size = counter.get() as f64;
        let count = (tarball_downloaded_size / file_size * 100.0) as usize;
        if let Ok(err) = error_channel_rx.try_recv() {
            return Err(anyhow!(err));
        }
        let v = vaild_rx.recv().ok();

        sender.send(InstallProgress::Pending(STEP2.to_string(), count, v))?;
        std::thread::sleep(refresh_interval);
        if download_done.load(Ordering::SeqCst) {
            break;
        }
    }
    let mut fake_counter = 0;

    info!("{}", STEP3);
    loop {
        sender.send(InstallProgress::Pending(
            STEP3.to_string(),
            fake_counter,
            None,
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

    info!("{}", STEP4);
    loop {
        let tarball_unpack_size = counter.get() as f64;
        let count = (tarball_unpack_size / file_size * 100.0) as usize;
        sender.send(InstallProgress::Pending(STEP4.to_string(), count, None))?;
        std::thread::sleep(refresh_interval);
        if extract_done.load(Ordering::SeqCst) {
            break;
        }
    }

    // GC the worker thread
    worker.join().unwrap();
    sha256sum_work.join().unwrap();
    // genfstab to file
    info!("Generating fstab ...");
    install::genfstab_to_file(partition, &tempdir, Path::new("/"))?;

    if disks::is_efi_booted() {
        info!("Generating fstab efi entry...");
        let esp_part = disks::find_esp_partition(partition.parent_path.as_ref().unwrap())?;
        install::genfstab_to_file(&esp_part, &tempdir, Path::new("/efi"))?;
    }
    let mut rng = thread_rng();
    let fake_counter: usize = rng.gen_range(0..100);

    sender.send(InstallProgress::Pending(
        STEP5.to_string(),
        fake_counter,
        None,
    ))?;
    info!("{}", STEP5);

    info!("Chroot to installed system ...");
    let escape_vector = install::get_dir_fd(PathBuf::from("/"))?;
    install::dive_into_guest(&mount_path_copy)?;

    info!("Running dracut ...");
    install::execute_dracut()?;

    let fake_counter: usize = rng.gen_range(0..100);
    sender.send(InstallProgress::Pending(
        STEP6.to_string(),
        fake_counter,
        None,
    ))?;
    info!("{}", STEP6);

    if disks::is_efi_booted() {
        info!("Installing grub to UEFI partition ...");
        install::execute_grub_install(None)?;
    } else {
        info!("Installing grub to MBR partition ...");
        install::execute_grub_install(Some(partition.parent_path.as_ref().unwrap()))?;
    };

    let fake_counter: usize = rng.gen_range(0..100);
    sender.send(InstallProgress::Pending(
        STEP7.to_string(),
        fake_counter,
        None,
    ))?;
    info!("{}", STEP7);

    info!("Generating SSH key ...");
    install::gen_ssh_key()?;

    info!("{}", STEP8);
    let fake_counter: usize = rng.gen_range(0..100);
    sender.send(InstallProgress::Pending(
        STEP8.to_string(),
        fake_counter,
        None,
    ))?;

    if use_swap {
        info!("Generating swapfile entry to fstab");
        install::write_swap_entry_to_fstab()?;
    }

    let tz = config.timezone.unwrap();
    info!("Setting timezone as {}", &tz);
    install::set_zoneinfo(&tz)?;

    let tc = config.tc.unwrap();
    info!("Setting hwclock (hardware clock) as {}", &tc);
    install::set_hwclock_tc(match tc.as_str() {
        "UTC" => true,
        "RTC" => false,
        _ => true,
    })?;

    let hostname = config.hostname.unwrap();
    info!("Setting hostname as {}", &hostname);
    install::set_hostname(&hostname)?;

    info!("Setting username and password ...");
    install::add_new_user(&config.user.clone().unwrap(), &config.password.unwrap())?;

    info!("Setting fullname ...");
    if config.full_name.is_some() && config.full_name != Some("".to_string().into()) {
        install::passwd_set_fullname(&config.full_name.unwrap(), &config.user.unwrap())?;
    }

    let locale = config.locale.as_ref().unwrap();
    info!("Setting locale as {}", locale);
    install::set_locale(locale)?;

    info!("Escaping chroot ...");
    install::escape_chroot(escape_vector.as_raw_fd())?;

    if disks::is_efi_booted() {
        info!("Unmounting EFI partition ...");
        install::umount_root_path(&efi_path)?;
    }

    info!("Copy log file to main partition");
    std::fs::copy(
        &logfile,
        tempdir.join("var").join("log").join(
            logfile
                .file_name()
                .ok_or_else(|| anyhow!("Can not get filename"))?,
        ),
    )?;

    info!("Removing bind mounts ...");
    install::remove_bind_mounts(&mount_path_copy)?;

    info!("Trying to swapoff ...");
    install::swapoff(&tempdir);

    info!("Unmounting main partition ...");
    install::umount_root_path(&mount_path_copy).ok();

    sender.send(InstallProgress::Finished)?;

    Ok(())
}
