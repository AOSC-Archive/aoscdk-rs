use anyhow::{anyhow, Result};

use disk_types::FileSystem;
use fstab_generate::BlockInfo;
use fstab_generate::PartitionID;
use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

const EFI_DETECT_PATH: &str = "/sys/firmware/efi";
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

pub fn format_partition(partition: &Partition) -> Result<()> {
    let default_fs = DEFAULT_FS_TYPE.to_owned();
    let fs_type = partition.fs_type.as_ref().unwrap_or(&default_fs);
    let mut command = Command::new(format!("mkfs.{}", fs_type));
    let cmd;
    let output;
    if fs_type == "ext4" {
        cmd = command.arg("-Fq");
    } else if fs_type == "vfat" {
        cmd = command.arg("-F32");
    } else {
        cmd = command.arg("-f");
    }
    output = cmd
        .arg(
            partition
                .path
                .as_ref()
                .ok_or_else(|| anyhow!("Path not found"))?,
        )
        .output()?;
    if !output.status.success() {
        return Err(anyhow!(
            "Failed to create filesystem: \n{}\n{}",
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        ));
    }

    Ok(())
}

pub fn fill_fs_type(part: &Partition) -> Partition {
    let mut new_part = part.clone();
    new_part.fs_type = Some(DEFAULT_FS_TYPE.to_string());

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
                let fs_type;
                if let Ok(type_) = part.get_geom().probe_fs() {
                    fs_type = Some(type_.name().to_owned());
                } else {
                    fs_type = None;
                }
                let path = part
                    .get_path()
                    .ok_or_else(|| anyhow!("Unable to get the device file for ESP partition"))?;
                return Ok(Partition {
                    path: Some(path.to_owned()),
                    parent_path: None,
                    size: 0,
                    fs_type,
                });
            }
        }
    }

    Err(anyhow!("ESP partition not found."))
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
                let fs_type;
                let geom_length: i64 = part.geom_length();
                let part_length = if geom_length < 0 {
                    0
                } else {
                    geom_length as u64
                };
                if let Ok(type_) = part.get_geom().probe_fs() {
                    fs_type = Some(type_.name().to_owned());
                } else {
                    fs_type = None;
                }
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

pub fn fstab_entries(partition: &Partition, target: &Path) -> Result<OsString> {
    let root_id =
        PartitionID::get_partuuid(target).ok_or_else(|| anyhow!("Could get partition uuid!"))?;
    let fs_type = partition
        .fs_type
        .as_ref()
        .ok_or_else(|| anyhow!("Could get partition Object!"))?;
    let (fs_type, option) = if fs_type.starts_with("vfat") {
        (FileSystem::Fat32, "defaults")
    } else if fs_type.starts_with("ext4") {
        (FileSystem::Ext4, "defaults")
    } else if fs_type.starts_with("swap") {
        (FileSystem::Swap, "sw")
    } else {
        return Err(anyhow!("Unsupport fs type!"));
    };
    let root = BlockInfo::new(root_id, fs_type, Some(target), option);
    let fstab = &mut OsString::new();
    root.write_entry(fstab);

    Ok(fstab.to_owned())
}
