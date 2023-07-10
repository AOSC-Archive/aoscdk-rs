use std::{
    os::unix::prelude::AsRawFd,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use indicatif::ProgressBar;
use log::{error, info};

use crate::{
    disks::{self, Partition},
    install::{self, is_acceptable_username, is_valid_hostname, umount_all},
    log::setup_logger,
    network::{self, fetch_mirrors, Mirror, VariantEntry},
};

use super::{begin_install, tui_main, AtomicBoolWrapper, InstallConfig, DEFAULT_EMPTY_SIZE};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
    #[clap(subcommand)]
    subcommand: DeployKitCliCommand,
}

#[derive(Subcommand, Debug)]
enum DeployKitCliCommand {
    /// Start Deploykit tui
    Tui(Tui),
    /// Install System
    Install(Box<InstallCommand>),
    /// List of mirror
    ListMirror(ListMirror),
    /// List of locale
    ListLocale(ListLocale),
    /// List of timezone
    ListTimezone(ListTimezone),
    /// List of tarball
    ListTarball(ListTarball),
}

#[derive(Parser, Debug)]
struct Tui;

#[derive(Parser, Debug)]
struct ListMirror;

#[derive(Parser, Debug)]
struct ListLocale;

#[derive(Parser, Debug)]
struct ListTimezone;

#[derive(Parser, Debug)]
struct ListTarball;

#[derive(Parser, Debug)]
struct InstallCommand {
    /// Select AOSC OS variant to install (e.g., Workstation, Server, Base)
    #[clap(long, default_value = "Base")]
    tarball: String,
    /// Set URL for download source
    #[clap(long, default_value = "https://repo.aosc.io/aosc-os")]
    mirror: String,
    /// Set target partition to install AOSC OS to (e.g., /dev/sda1)
    #[clap(long)]
    path: String,
    /// Set name of the default user
    #[clap(long)]
    user: String,
    /// Set password for default user
    #[clap(long)]
    password: String,
    /// Set device hostname
    #[clap(long, default_value = "aosc")]
    hostname: String,
    /// Set default timezone
    #[clap(long, default_value = "UTC")]
    timezone: String,
    /// Set default locale (affects display language, units, time/date format etc.)
    #[clap(long, default_value = "C.UTF-8")]
    locale: String,
    /// Toggle using RTC (real time clock) time as local time
    #[clap(long, action = clap::ArgAction::SetTrue)]
    use_rtc: bool,
    /// Disable swapfile
    #[clap(long, conflicts_with = "swap_size", action = clap::ArgAction::SetTrue)]
    no_swap: bool,
    /// Set custom swapfile size
    #[clap(long, conflicts_with = "no_swap")]
    swap_size: Option<f64>,
}

pub fn execute(args: Args) -> Result<()> {
    match args.subcommand {
        DeployKitCliCommand::Tui(Tui) => tui_main(),
        DeployKitCliCommand::Install(ic) => start_install(*ic)?,
        DeployKitCliCommand::ListMirror(ListMirror) => list_mirror()?,
        DeployKitCliCommand::ListLocale(ListLocale) => list_locale()?,
        DeployKitCliCommand::ListTimezone(ListTimezone) => list_timezone()?,
        DeployKitCliCommand::ListTarball(ListTarball) => list_tarball()?,
    }

    Ok(())
}

fn list_mirror() -> Result<()> {
    let recipe = network::fetch_recipe()?;
    let mirrors = fetch_mirrors(&recipe);
    for i in mirrors {
        println!("{:<40}{}", i.name, i.url);
    }

    Ok(())
}

fn list_locale() -> Result<()> {
    let locale_list = install::get_locale_list()?;
    for i in locale_list {
        println!("{i}");
    }

    Ok(())
}

fn list_timezone() -> Result<()> {
    let timezone_list = install::get_zoneinfo_list()?;
    for i in timezone_list {
        println!("{i}");
    }

    Ok(())
}

fn list_tarball() -> Result<()> {
    let variants = network::get_variants()?;
    for i in variants {
        println!("{}", i.name);
    }

    Ok(())
}

fn get_variant(tarball: &str) -> Result<VariantEntry> {
    let variants = network::get_variants()?;

    let index = variants
        .iter()
        .position(|x| x.name.to_lowercase() == tarball.to_lowercase());
    if let Some(index) = index {
        return Ok(variants[index].to_owned());
    }

    Err(anyhow!(
        "Installer could not find tarball for specified variant {}.\nPlease refer to the `aoscdk-rs list-tarball` output for a list of available tarballs.",
        tarball
    ))
}

fn get_partition(path: &str, variant: &VariantEntry) -> Result<Partition> {
    let required_size = variant.install_size + variant.size;
    if cfg!(debug_assertions) {
        disks::right_combine(Some(&PathBuf::from("/dev/loop20")))?;

        return Ok(Partition {
            fs_type: Some("ext4".to_string()),
            path: Some(PathBuf::from("/dev/loop20p1")),
            parent_path: Some(PathBuf::from("/dev/loop20")),
            size: 53687091200,
        });
    }
    let path = Path::new(path);
    let list_part = disks::list_partitions();
    let index = list_part
        .iter()
        .position(|x| x.path == Some(path.to_path_buf()));
    if let Some(index) = index {
        let partition = list_part[index].to_owned();
        if partition.size < required_size + variant.size {
            let s = format!(
                "The specified partition does not contain enough space to install AOSC OS release!\n\nAvailable space: {:.3}GiB\nRequired space: {:.3}GiB", 
                partition.size as f32 / 1024.0 / 1024.0 / 1024.0,
                required_size as f32 / 1024.0 / 1024.0 / 1024.0
            );
            return Err(anyhow!(s));
        }
        let partition = disks::fill_fs_type(&partition, false);
        disks::right_combine(partition.parent_path.as_deref())?;

        return Ok(partition);
    }

    Err(anyhow!(
        "Installer could not find the specified partition: {}\nDid you partition your target disk?",
        path.display()
    ))
}

fn get_mirror(mirror: &str) -> Mirror {
    let s = "cli_usage";
    let mirror = if mirror.ends_with('/') {
        mirror.to_string()
    } else {
        format!("{mirror}/")
    };

    Mirror {
        name: s.to_string(),
        name_tr: s.to_string(),
        loc: s.to_string(),
        loc_tr: s.to_string(),
        url: mirror,
    }
}

fn get_swap(
    swap_size: Option<f64>,
    partition: &Partition,
    variant: &VariantEntry,
) -> Result<(bool, f64, bool)> {
    let result = if let Some(swap_size) = swap_size {
        let size = swap_size * 1024.0 * 1024.0 * 1024.0;
        let is_hibernation = disks::is_enable_hibernation(size)?;

        (true, size, is_hibernation)
    } else {
        let size = disks::get_recommand_swap_size()?;

        if partition.size as f64
            > size + variant.install_size as f64 + variant.size as f64 - DEFAULT_EMPTY_SIZE as f64
        {
            (true, size, true)
        } else {
            (false, size, false)
        }
    };

    Ok(result)
}

fn start_install(ic: InstallCommand) -> Result<()> {
    let logfile = setup_logger(true)?;

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let variant = get_variant(&ic.tarball)?;
    let partition = get_partition(&ic.path, &variant)?;
    let mirror = get_mirror(&ic.mirror);
    let tc = if ic.use_rtc { "RTC" } else { "UTC" };
    let (use_swap, swap_size, is_hibernation) = get_swap(ic.swap_size, &partition, &variant)?;

    if !is_valid_hostname(&ic.hostname) {
        return Err(anyhow!("hostname {} is not valid!", ic.hostname));
    }

    if !is_acceptable_username(&ic.user) {
        return Err(anyhow!("username {} is not valid!", ic.user));
    }

    let install_config = InstallConfig {
        variant: Some(Arc::new(variant)),
        partition: Some(Arc::new(partition)),
        mirror: Some(Arc::new(mirror)),
        user: Some(Arc::new(ic.user)),
        full_name: None,
        password: Some(Arc::new(ic.password)),
        hostname: Some(ic.hostname),
        locale: Some(Arc::new(ic.locale)),
        timezone: Some(Arc::new(ic.timezone)),
        tc: Some(Arc::new(tc.to_string())),
        use_swap: Arc::new(AtomicBoolWrapper {
            v: AtomicBool::new(!ic.no_swap && use_swap),
        }),
        swap_size: Arc::new(Some(swap_size)),
        is_hibernation: Arc::new(AtomicBoolWrapper {
            v: AtomicBool::new(is_hibernation),
        }),
        root_password: None
    };

    let root_fd = install::get_dir_fd(PathBuf::from("/"))?.as_raw_fd();
    let (tx, rx) = std::sync::mpsc::channel();

    let tempdir = tempfile::Builder::new()
        .prefix(".dkmount")
        .tempdir()
        .expect("Installer failed to create temporary file for the download process.")
        .into_path();

    let tempdir_clone = tempdir.clone();
    let tempdir_clone_2 = tempdir.clone();
    ctrlc::set_handler(move || {
        info!("User request to exit the installer");
        umount_all(&tempdir, root_fd);
        r.store(false, Ordering::SeqCst);
    }).expect("Installer could not initialize SIGINT handler.\n\nPlease restart your installation environment.");

    let install_thread =
        thread::spawn(move || begin_install(tx, install_config, tempdir_clone, logfile));
    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(Duration::from_millis(50));

    loop {
        if !running.load(Ordering::SeqCst) {
            return Err(anyhow!("AOSC OS installation has been aborted."));
        }
        if let Ok(progress) = rx.recv() {
            match progress {
                super::InstallProgress::Pending(msg, pct) => {
                    bar.set_message(format!("{msg} ({pct}/100)"));
                }
                super::InstallProgress::Finished => {
                    bar.finish_with_message("AOSC OS installation has successfully completed! Good luck to you, Dungeon Master :)");
                    return Ok(());
                }
            }
        } else {
            let err = install_thread.join().map_err(|_| anyhow!("Installer has encountered an unexpected error. Please restart your installation environment."))?.unwrap_err();

            error!("{}", err);
            umount_all(&tempdir_clone_2, root_fd);
            return Err(err);
        }
    }
}

#[test]
fn test() {
    dbg!(list_tarball().unwrap());
}
