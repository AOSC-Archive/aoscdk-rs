use std::{
    path::{Path, PathBuf},
    sync::{atomic::{AtomicBool, Ordering}, Arc},
    thread, os::unix::prelude::AsRawFd
};

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use indicatif::ProgressBar;
use tempfile::TempDir;

use crate::{
    disks::{self, Partition},
    install::{self, umount_all},
    network::{self, fetch_mirrors, Mirror, VariantEntry},
};

use super::{begin_install, InstallConfig};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
    #[clap(subcommand)]
    subcommand: DeployKitCliCommand,
}

#[derive(Subcommand, Debug)]
enum DeployKitCliCommand {
    /// Install System
    Install(Box<InstallCommand>),
    /// List of mirror
    ListMirror(ListMirror),
    /// List of locale
    ListLocale(ListLocale),
    /// List of timezone
    ListTimezone(ListTimezone),
}

#[derive(Parser, Debug)]
struct ListMirror;

#[derive(Parser, Debug)]
struct ListLocale;

#[derive(Parser, Debug)]
struct ListTimezone;

#[derive(Parser, Debug)]
struct InstallCommand {
    /// Variant name, like: Base, KDE ...
    #[clap(long, default_value = "Base")]
    tarball: String,
    /// Mirror url
    #[clap(long, default_value = "https://repo.aosc.io/aosc-os")]
    mirror: String,
    /// Install AOSC OS to path, like: /dev/sda1
    #[clap(long)]
    path: String,
    /// Set username
    #[clap(long)]
    user: String,
    /// Set password
    #[clap(long)]
    password: String,
    /// Set hostname
    #[clap(long, default_value = "aosc")]
    hostname: String,
    /// Set timezone
    #[clap(long, default_value = "Asia/Shanghai")]
    timezone: String,
    /// Set locale
    #[clap(long, default_value = "C.UTF-8")]
    locale: String,
    /// Use RTC as system time
    #[clap(long)]
    use_rtc: bool,
    /// Dont use swap
    #[clap(long, conflicts_with = "swap-size")]
    no_swap: bool,
    /// Custom swap size
    #[clap(long)]
    swap_size: Option<f64>,
}

pub fn execute(args: Args) -> Result<()> {
    match args.subcommand {
        DeployKitCliCommand::Install(ic) => start_install(*ic)?,
        DeployKitCliCommand::ListMirror(ListMirror) => list_mirror()?,
        DeployKitCliCommand::ListLocale(ListLocale) => list_locale()?,
        DeployKitCliCommand::ListTimezone(ListTimezone) => list_timezone()?,
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
        println!("{}", i);
    }

    Ok(())
}

fn list_timezone() -> Result<()> {
    let timezone_list = install::get_zoneinfo_list()?;
    for (continent, city) in timezone_list {
        for c in city {
            println!("{}/{}", continent, c);
        }
    }

    Ok(())
}

fn get_variant(tarball: &str) -> Result<VariantEntry> {
    let recipe = network::fetch_recipe()?;
    let variants = network::find_variant_candidates(recipe)?;
    let index = variants
        .iter()
        .position(|x| x.name == tarball || x.name.to_lowercase() == tarball);
    if let Some(index) = index {
        return Ok(variants[index].to_owned());
    }

    Err(anyhow!(
        "Could not find variant at tarball name: {}",
        tarball
    ))
}

fn get_partition(path: &str, variant: &VariantEntry) -> Result<Partition> {
    let required_size = variant.install_size;
    if cfg!(debug_assertions) {
        return Ok(Partition {
            fs_type: Some("ext4".to_string()),
            path: Some(PathBuf::from("/dev/loop0p1")),
            parent_path: Some(PathBuf::from("/dev/loop0")),
            size: required_size + 10 * 1024 * 1024 * 1024,
        });
    }
    let path = Path::new(path);
    let list_part = disks::list_partitions();
    let index = list_part
        .iter()
        .position(|x| x.path == Some(path.to_path_buf()));
    if let Some(index) = index {
        let partition = list_part[index].to_owned();
        if partition.size < required_size {
            let s = format!(
                "The selected partition is not enough to install this tarball!\nCurrent disk size: {:.3}GiB\nDisk size required: {:.3}GiB", 
                partition.size as f32 / 1024.0 / 1024.0 / 1024.0,
                required_size as f32 / 1024.0 / 1024.0 / 1024.0
            );
            return Err(anyhow!(s));
        }
        let partition = disks::fill_fs_type(&partition, false);

        return Ok(partition);
    }

    Err(anyhow!(
        "Could not find partition in path: {}",
        path.display()
    ))
}

fn get_mirror(mirror: &str) -> Mirror {
    let s = "cli_usage";
    let mirror = if mirror.ends_with('/') {
        mirror.to_string()
    } else {
        format!("{}/", mirror)
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

        if partition.size > size as u64 + variant.install_size {
            (true, size, true)
        } else {
            (false, size, false)
        }
    };

    Ok(result)
}

fn start_install(ic: InstallCommand) -> Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let variant = get_variant(&ic.tarball)?;
    let partition = get_partition(&ic.path, &variant)?;
    let mirror = get_mirror(&ic.mirror);
    let (continent, city) = ic
        .timezone
        .split_once('/')
        .ok_or_else(|| anyhow!("Can not parse timezone!"))?;
    let tc = if ic.use_rtc { "RTC" } else { "UTC" };
    let (use_swap, swap_size, is_hibernation) = get_swap(ic.swap_size, &partition, &variant)?;

    let install_config = InstallConfig {
        variant: Some(Arc::new(variant)),
        partition: Some(Arc::new(partition)),
        mirror: Some(Arc::new(mirror)),
        user: Some(Arc::new(ic.user)),
        password: Some(Arc::new(ic.password)),
        hostname: Some(ic.hostname),
        locale: Some(Arc::new(ic.locale)),
        continent: Some(Arc::new(continent.to_string())),
        city: Some(Arc::new(city.to_string())),
        tc: Some(Arc::new(tc.to_string())),
        use_swap: Arc::new(AtomicBool::new(!ic.no_swap && use_swap)),
        swap_size: Arc::new(Some(swap_size)),
        is_hibernation: Arc::new(AtomicBool::new(is_hibernation)),
    };
    let root_fd = install::get_dir_fd(PathBuf::from("/"))?.as_raw_fd();
    let (tx, rx) = std::sync::mpsc::channel();
    let tempdir = TempDir::new()
        .expect("Unable to create temporary directory")
        .into_path();
    let tempdir_clone = tempdir.clone();
    let tempdir_clone_2 = tempdir.clone();
    ctrlc::set_handler(move || {
        umount_all(&tempdir, root_fd);
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting SIGINT handler.");
    let install_thread =
        thread::spawn(move || begin_install(tx, install_config, tempdir_clone));
    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(50);
    loop {
        if !running.load(Ordering::SeqCst) {
            return Err(anyhow!("User interrup!"));
        }
        if let Ok(progress) = rx.recv() {
            match progress {
                super::InstallProgress::Pending(msg, pct) => {
                    bar.set_message(format!("{} ({}/100)", msg, pct));
                }
                super::InstallProgress::Finished => {
                    bar.finish_with_message("Install finished! Changed your house to dungeon.");
                    return Ok(());
                }
            }
        } else {
            let err = install_thread.join().map_err(|_| anyhow!("Join thread failed! Why"))?.unwrap_err();
            umount_all(&tempdir_clone_2, root_fd);
            return Err(err);
        }
    }
}
