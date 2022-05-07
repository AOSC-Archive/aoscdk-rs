use anyhow::{anyhow, Result};

use disk_types::FileSystem;
use fstab_generate::BlockInfo;
use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

const EFI_DETECT_PATH: &str = "/sys/firmware/efi";
pub(crate) const ALLOWED_FS_TYPE: &[&str] = &["ext4", "xfs", "btrfs", "f2fs"];
const DEFAULT_FS_TYPE: &str = "ext4";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Partition {
    pub path: Option<PathBuf>,
    pub parent_path: Option<PathBuf>,
    pub fs_type: Option<String>,
    pub size: u64,
}

#[inline]
pub fn is_efi_booted() -> bool {
    Path::new(EFI_DETECT_PATH).is_dir()
}

pub fn get_recommended_fs_type(type_: &str) -> &str {
    for i in ALLOWED_FS_TYPE {
        if *i == type_ {
            return i;
        }
    }

    DEFAULT_FS_TYPE
}

pub fn format_partition(partition: &Partition) -> Result<()> {
    let default_fs = DEFAULT_FS_TYPE.to_owned();
    let fs_type = partition.fs_type.as_ref().unwrap_or(&default_fs);
    let mut command = Command::new(format!("mkfs.{}", fs_type));
    let cmd;

    if fs_type == "ext4" {
        cmd = command.arg("-Fq");
    } else if fs_type == "vfat" {
        cmd = command.arg("-F32");
    } else {
        cmd = command.arg("-f");
    }
    let output = cmd
        .arg(
            partition
                .path
                .as_ref()
                .ok_or_else(|| anyhow!("AOSC OS Installer cannot find the specified partition.\nDid you partition your target disk?"))?,
        )
        .output()?;
    if !output.status.success() {
        return Err(anyhow!(
            "AOSC OS Installer failed to format the specified partition: \n{}\n{}",
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        ));
    }

    Ok(())
}

pub fn fill_fs_type(part: &Partition, use_ext4: bool) -> Partition {
    let mut new_part = part.clone();
    let new_fs_type: String;
    if let Some(fs_type) = new_part.fs_type.clone() {
        if !use_ext4 {
            new_fs_type = get_recommended_fs_type(&fs_type).to_string();
        } else {
            new_fs_type = DEFAULT_FS_TYPE.to_string();
        }
    } else {
        new_fs_type = DEFAULT_FS_TYPE.to_string();
    }
    new_part.fs_type = Some(new_fs_type);

    new_part
}

pub fn find_esp_partition(device_path: &Path) -> Result<Partition> {
    let mut device = libparted::Device::get(device_path)?;
    if let Ok(disk) = libparted::Disk::new(&mut device) {
        for mut part in disk.parts() {
            if part.num() < 0 {
                continue;
            }
            if part.get_flag(libparted::PartitionFlag::PED_PARTITION_ESP) {
                let fs_type = if let Ok(type_) = part.get_geom().probe_fs() {
                    Some(type_.name().to_owned())
                } else {
                    None
                };
                let path = part.get_path().ok_or_else(|| {
                    anyhow!("AOSC OS Installer could not detect the EFI system partition.")
                })?;
                return Ok(Partition {
                    path: Some(path.to_owned()),
                    parent_path: None,
                    size: 0,
                    fs_type,
                });
            }
        }
    }

    Err(anyhow!(
        "AOSC OS Installer could not detect the EFI system partition."
    ))
}

pub fn list_partitions() -> Vec<Partition> {
    let mut partitions: Vec<Partition> = Vec::new();
    for mut device in libparted::Device::devices(true) {
        let device_path = device.path().to_owned();
        let sector_size: u64 = device.sector_size();
        if let Ok(disk) = libparted::Disk::new(&mut device) {
            for mut part in disk.parts() {
                if part.num() < 0 {
                    continue;
                }
                let geom_length: i64 = part.geom_length();
                let part_length = if geom_length < 0 {
                    0
                } else {
                    geom_length as u64
                };
                let fs_type = if let Ok(type_) = part.get_geom().probe_fs() {
                    Some(type_.name().to_owned())
                } else {
                    None
                };
                partitions.push(Partition {
                    path: part.get_path().map(|path| path.to_owned()),
                    parent_path: Some(device_path.clone()),
                    size: sector_size * part_length,
                    fs_type,
                });
            }
        }
    }

    partitions
}

fn partition_is_gpt(device_path: Option<&PathBuf>) -> Result<bool> {
    let target = device_path.ok_or_else(|| anyhow!("AOSC OS Installer cannot detect the corresponding device file for the specified partition!"))?;
    let mut device = std::fs::File::open(target)?;
    if gptman::GPT::find_from(&mut device).is_ok() {
        return Ok(true);
    }

    Ok(false)
}

#[cfg(not(target_arch = "powerpc64"))]
pub fn right_combine(device_path: Option<&PathBuf>) -> Result<()> {
    let is_gpt = partition_is_gpt(device_path)?;
    let is_efi_booted = is_efi_booted();
    let gpt_mbr_s = if is_gpt { "GPT" } else { "MBR" };
    let bios_efi_s = if is_efi_booted { "EFI" } else { "BIOS" };
    let right = if is_efi_booted { "GPT" } else { "BIOS" };
    if (is_gpt && is_efi_booted) || (!is_efi_booted && !is_gpt) {
        return Ok(());
    }

    Err(anyhow!("AOSC OS Installer has detected an unsupported partition map for your device ({} partition map on a {}-based device). Please use a {} partition map for your {}-based device.", gpt_mbr_s, bios_efi_s, right, bios_efi_s))
}

#[cfg(target_arch = "powerpc64")]
pub fn right_combine(device_path: Option<&PathBuf>) -> Result<()> {
    use crate::network;
    let is_gpt = partition_is_gpt(device_path)?;
    let arch_name = network::get_arch_name();

    if arch_name == Some("ppc64el") && !is_gpt {
        return Err(anyhow!("AOSC OS Installer detected an unsupported partition map for your device. Please use a GPT partition map for your POWER/CHRP-based device."));
    }

    Ok(())
}

pub fn fstab_entries(
    device_path: Option<&PathBuf>,
    fs_type: &str,
    mount_path: Option<&Path>,
) -> Result<OsString> {
    let target = device_path.ok_or_else(|| anyhow!("AOSC OS Installer cannot detect the corresponding device file for the specified partition!"))?;
    let (fs_type, option) = match fs_type {
        "vfat" | "fat16" | "fat32" => (FileSystem::Fat32, "defaults"),
        "ext4" => (FileSystem::Ext4, "defaults"),
        "btrfs" => (FileSystem::Btrfs, "defaults"),
        "xfs" => (FileSystem::Xfs, "defaults"),
        "f2fs" => (FileSystem::F2fs, "defaults"),
        "swap" => (FileSystem::Swap, "sw"),
        _ => return Err(anyhow!("Unsupported filesystem type!")),
    };
    let root_id = BlockInfo::get_partition_id(target, fs_type).ok_or_else(|| {
        anyhow!(
            "AOSC OS Installer cannot obtain partition UUID for {}!",
            target.display()
        )
    })?;
    let root = BlockInfo::new(root_id, fs_type, mount_path, option);
    let fstab = &mut OsString::new();
    root.write_entry(fstab);

    Ok(fstab.to_owned())
}

pub fn get_recommand_swap_size() -> Result<f64> {
    // Get men (GiB)
    let men = (sys_info::mem_info()?.total / 1024 / 1024) as f64;
    let swap_size = if men <= 5.0 {
        1.3 * men + 0.7
    } else if men > 5.0 && men <= 32.0 {
        1.1543 * men + 1.36328
    } else {
        1.009945 * men + 16.087529
    };
    // Swap size GiB to iB
    let swap_size = swap_size.round() * 1024.0 * 1024.0 * 1024.0;

    Ok(swap_size)
}

pub fn is_enable_hibernation(custom_size: f64) -> Result<bool> {
    // Get men (iB)
    let men = (sys_info::mem_info()?.total * 1024) as f64;
    let recommand_size = get_recommand_swap_size()?;
    let no_hibernation_size = recommand_size - men;
    if custom_size >= no_hibernation_size && custom_size < recommand_size {
        return Ok(false);
    } else if custom_size >= recommand_size {
        return Ok(true);
    }

    // Round back to GiB for display message.
    Err(anyhow!("The specified swapfile size is too small, AOSC OS Installer recommends at least {} GiB for your device.", (recommand_size / 1024.0 / 1024.0 / 1024.0).round()))
}

#[test]
fn test_fs_recommendation() {
    assert_eq!(get_recommended_fs_type("btrfs"), "btrfs");
    assert_eq!(get_recommended_fs_type("ext2"), "ext4");
}
