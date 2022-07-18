use anyhow::{anyhow, Result};

use disk_types::FileSystem;
use fstab_generate::BlockInfo;
use libparted::Constraint;
use libparted::DiskType;
use libparted::FileSystemType;
use libparted::IsZero;
use libparted::PartitionType;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::ffi::CStr;
use std::ffi::OsString;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use crate::frontend::RESCUEKIT_SIZE;

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
                .ok_or_else(|| anyhow!("Installer could not find the specified partition.\nDid you partition your target disk?"))?,
        )
        .output()?;
    if !output.status.success() {
        return Err(anyhow!(
            "Installer failed to format the specified partition: \n{}\n{}",
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
                    anyhow!("Installer could not detect the EFI system partition.")
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
        "Installer could not detect the EFI system partition."
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
                let part_length = get_part_length(&part);
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

fn get_part_length(part: &libparted::Partition) -> u64 {
    let geom_length: i64 = part.geom_length();
    let part_length = if geom_length < 0 {
        0
    } else {
        geom_length as u64
    };

    part_length
}

fn get_partition_table_type(device_path: Option<&PathBuf>) -> Result<String> {
    fn cvt<T: IsZero>(t: T) -> io::Result<T> {
        if t.is_zero() {
            Err(io::Error::last_os_error())
        } else {
            Ok(t)
        }
    }

    let target = device_path.ok_or_else(|| {
        anyhow!(
            "Installer could not detect the corresponding block device node for the specified partition!"
        )
    })?;
    let device = libparted::Device::new(target)?;
    let partition_t = cvt(unsafe { libparted_sys::ped_disk_probe(device.ped_device()) });
    if let Ok(partition_t) = partition_t {
        if let Ok(partition_t_name) = unsafe { cvt((*partition_t).name) } {
            let partition_t = unsafe { CStr::from_ptr(partition_t_name) };
            let partition_t = partition_t.to_str()?.to_string();

            return Ok(partition_t);
        }
    }

    Err(anyhow!(
        "Installer does support the specified partition map for your device."
    ))
}

#[cfg(not(target_arch = "powerpc64"))]
pub fn new_partition_table() -> Result<()> {
    let t = if is_efi_booted() { "gpt" } else { "msdos" };
    for mut i in libparted::Device::devices(true) {
        if libparted::Disk::new(&mut i).is_err() {
            let disk = libparted::Disk::new_fresh(
                &mut i,
                DiskType::get(t).ok_or_else(|| anyhow!("Installer does support the specified partition map for your device (supported partition maps: GPT (for EFI systems) and MBR/MS-DOS (for BIOS systems)."))?,
            );
            if let Ok(mut disk) = disk {
                disk.commit_to_dev().ok();
            }
        }
    }

    Ok(())
}

#[cfg(target_arch = "powerpc64")]
pub fn new_partition_table() -> Result<()> {
    use crate::network;

    let arch_name = network::get_arch_name();
    for mut i in libparted::Device::devices(true) {
        if libparted::Disk::new(&mut i).is_err() && arch_name == Some("ppc64el") {
            let disk = libparted::Disk::new_fresh(
                &mut i,
                DiskType::get("gpt").ok_or_else(|| anyhow!("Installer does support the specified partition map for your device (supported partition map: GPT)."))?,
            );
            if let Ok(mut disk) = disk {
                disk.commit_to_dev().ok();
            }
        }
    }

    Ok(())
}

#[cfg(not(target_arch = "powerpc64"))]
pub fn right_combine(device_path: Option<&PathBuf>) -> Result<()> {
    let partition_table_t = get_partition_table_type(device_path)?;
    let is_efi_booted = is_efi_booted();
    let bios_efi_s = if is_efi_booted { "EFI" } else { "BIOS" };
    let right = if is_efi_booted { "GPT" } else { "BIOS" };
    if (partition_table_t == "gpt" && is_efi_booted)
        || (partition_table_t == "msdos" && !is_efi_booted)
    {
        return Ok(());
    }

    Err(anyhow!("Installer detected an unsupported partition map for your device ({} partition map on a {}-based device). Please use a {} partition map for your {}-based device.", partition_table_t, bios_efi_s, right, bios_efi_s))
}

#[cfg(target_arch = "powerpc64")]
pub fn right_combine(device_path: Option<&PathBuf>) -> Result<()> {
    use crate::network;
    let partition_table_t = get_partition_table_type(device_path)?;
    let arch_name = network::get_arch_name();

    if arch_name == Some("ppc64el") && partition_table_t != "gpt" {
        return Err(anyhow!("Installer detected an unsupported partition map for your device. Please use a GPT partition map for your POWER/CHRP-based device."));
    }

    Ok(())
}

pub fn fstab_entries(
    device_path: Option<&PathBuf>,
    fs_type: &str,
    mount_path: Option<&Path>,
) -> Result<OsString> {
    let target = device_path.ok_or_else(|| {
        anyhow!(
            "Installer could not detect the corresponding device file for the specified partition!"
        )
    })?;
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
            "Installer could not obtain partition UUID for {}!",
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
    Err(anyhow!("The specified swapfile size is too small, AOSC OS recommends at least {} GiB for your device.", (recommand_size / 1024.0 / 1024.0 / 1024.0).round()))
}

pub fn install_rescuekit_part(part: &Partition) -> Result<(Partition, Partition)> {
    let part_path = part
        .path
        .as_ref()
        .ok_or_else(|| anyhow!("Can not get pre-install system partition!"))?;
    let parent_path = part
        .parent_path
        .as_ref()
        .ok_or_else(|| anyhow!("Can not get block device!"))?;

    let mut dev = libparted::Device::new(&parent_path)?;
    let disk = libparted::Disk::new(&mut dev)?;

    let mut part = None;

    let parts = disk.parts();

    for i in parts {
        if Some(part_path.as_path()) == i.get_path() {
            part = Some(i)
        }
    }

    let dev = libparted::Device::new(parent_path)?;

    let sector_size = dev.sector_size();

    let mut dev = libparted::Device::new(parent_path)?;
    let mut disk = libparted::Disk::new(&mut dev)?;

    let mut part = part.ok_or_else(|| anyhow!("Can not get part!"))?;
    let part_fs = part.fs_type_name().unwrap_or("ext4");
    let pre_size = sector_size * get_part_length(&part) - RESCUEKIT_SIZE;
    let part_start = part.geom_start();
    let part_end = part_start + (pre_size / sector_size - 1) as i64;

    let num = part.num();
    disk.remove_partition_by_number(num.try_into()?)?;
    disk.commit_to_dev()?;

    let mut new_main_part = libparted::Partition::new(
        &disk,
        PartitionType::PED_PARTITION_NORMAL,
        FileSystemType::get(part_fs).as_ref(),
        part_start,
        part_end,
    )?;

    disk.add_partition(
        &mut new_main_part,
        &Constraint::new_from_max(&part.get_geom())?,
    )?;

    disk.commit_to_dev()?;

    let new_part_fs = new_main_part.fs_type_name().unwrap_or("ext4").to_string();

    let mut rescuekit_part = libparted::Partition::new(
        &disk,
        PartitionType::PED_PARTITION_NORMAL,
        FileSystemType::get("ext4").as_ref(),
        part_end + 1,
        part_end + 1 + (RESCUEKIT_SIZE / sector_size - 1) as i64,
    )?;

    let rescue_goem = rescuekit_part.get_geom();

    disk.add_partition(
        &mut rescuekit_part,
        &Constraint::new_from_max(&rescue_goem)?,
    )?;

    disk.commit_to_dev()?;

    let new_main_length = get_part_length(&new_main_part);

    let main_part = Partition {
        path: new_main_part.get_path().map(|path| path.to_owned()),
        parent_path: Some(parent_path.clone()),
        size: new_main_length * sector_size,
        fs_type: Some(new_part_fs.to_string()),
    };

    let rescuekit_part = Partition {
        path: rescuekit_part.get_path().map(|path| path.to_owned()),
        parent_path: Some(parent_path.clone()),
        size: 5 * 1024 * 1024 * 1024,
        fs_type: Some("ext4".to_string()),
    };

    Ok((main_part, rescuekit_part))
}

#[test]
fn test_fs_recommendation() {
    assert_eq!(get_recommended_fs_type("btrfs"), "btrfs");
    assert_eq!(get_recommended_fs_type("ext2"), "ext4");
}
