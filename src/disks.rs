use anyhow::bail;
use anyhow::{anyhow, Result};

use disk_types::BlockDeviceExt;
use disk_types::FileSystem;
use disk_types::PartitionExt;
use disk_types::PartitionType;
use fstab_generate::BlockInfo;
use libparted::Device;
use libparted::Disk;
use libparted::DiskType;
use libparted::FileSystemType;
use libparted::Geometry;
use libparted::IsZero;
use libparted::Partition as PedPartition;
use libparted_sys::PedPartitionFlag;
use libparted_sys::PedPartitionType;
use log::error;
use log::info;
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
use std::ffi::OsString;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

const EFI_DETECT_PATH: &str = "/sys/firmware/efi";
pub const ALLOWED_FS_TYPE: &[&str] = &["ext4", "xfs"];
const DEFAULT_FS_TYPE: &str = "ext4";

const MBR_NON_PRIMARY_PART_ERROR: &str = r#"Installer has detected that you are attempting to install AOSC OS on an MBR extended partition. This is not allowed as it may cause startup issues.

Please select a primary partition instead."#;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Partition {
    pub path: Option<PathBuf>,
    pub parent_path: Option<PathBuf>,
    pub fs_type: Option<String>,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DkDerive {
    pub path: PathBuf,
    pub model: String,
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
    let mut command = Command::new(format!("mkfs.{fs_type}"));
    let cmd;

    if fs_type == "ext4" {
        cmd = command.arg("-Fq");
    } else if fs_type == "vfat" {
        cmd = command.arg("-F32");
    } else {
        cmd = command.arg("-f");
    }

    info!("{cmd:?}");
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

pub fn list_devices() -> Vec<Device<'static>> {
    libparted::Device::devices(true).collect()
}

pub fn list_partitions(device_path: Option<PathBuf>) -> Vec<Partition> {
    let mut partitions: Vec<Partition> = Vec::new();
    if let Some(device_path) = device_path {
        if let Ok(dev) = Device::new(&device_path) {
            let sector_size = dev.sector_size();
            loop_device_get_parts(dev, &mut partitions, device_path, sector_size);
        }
    } else {
        for device in libparted::Device::devices(true) {
            let device_path = device.path().to_owned();
            let sector_size = device.sector_size();
            loop_device_get_parts(device, &mut partitions, device_path, sector_size);
        }
    }

    partitions
}

pub fn device_is_empty(dev: &Path) -> Result<bool> {
    let mut dev = libparted::Device::new(dev)?;
    let disk = libparted::Disk::new(&mut dev)?;
    let mut parts = disk.parts();

    Ok(parts.all(|x| x.get_path().is_none()))
}

fn loop_device_get_parts(
    mut device: Device<'_>,
    partitions: &mut Vec<Partition>,
    device_path: PathBuf,
    sector_size: u64,
) {
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

fn cvt<T: IsZero>(t: T) -> io::Result<T> {
    if t.is_zero() {
        Err(io::Error::last_os_error())
    } else {
        Ok(t)
    }
}

fn get_partition_table_type(device_path: Option<&Path>) -> Result<String> {
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

pub fn mbr_is_primary_partition(
    device_path: Option<&Path>,
    part_path: Option<&Path>,
) -> Result<()> {
    let partition_t = get_partition_table_type(device_path)?;

    if partition_t != "msdos" {
        return Ok(());
    }

    for mut device in libparted::Device::devices(true) {
        if let Ok(disk) = libparted::Disk::new(&mut device) {
            let parts = disk.parts().collect::<Vec<_>>();
            let index = parts
                .iter()
                .position(|x| x.get_path() == part_path)
                .ok_or_else(|| anyhow!("Can not find select partition!"))?;

            let part_type = parts[index].type_get_name();

            if part_type != "primary" {
                return Err(anyhow!(MBR_NON_PRIMARY_PART_ERROR));
            } else {
                return Ok(());
            }
        }
    }

    Ok(())
}

#[cfg(not(target_arch = "powerpc64"))]
pub fn right_combine(device_path: Option<&Path>) -> Result<()> {
    let partition_table_t = get_partition_table_type(device_path)?;
    let is_efi_booted = is_efi_booted();
    if (partition_table_t == "gpt" && is_efi_booted)
        || (partition_table_t == "msdos" && !is_efi_booted)
    {
        return Ok(());
    }

    let s = if std::env::var("DISPLAY").is_ok() {
        "Open GParted"
    } else {
        "Open Shell"
    };

    if partition_table_t == "gpt" && !is_efi_booted {
        bail!("Error: Installer has detected that you are using an unsupported partition map. Please select \"{s}\" to reset your partition table - for PC BIOS systems, please use the DOS/MBR partition map.")
    } else if partition_table_t == "msdos" && is_efi_booted {
        bail!("Error: Installer has detected that you are using an unsupported partition map. Please select \"{s}\" to reset your partition table - for UEFI systems, please use the GPT partition map.")
    } else {
        bail!("Error: Unsupported combination of firmware/partition map detected. Installer cannot install AOSC OS on this system.")
    }
}

#[cfg(target_arch = "powerpc64")]
pub fn right_combine(device_path: Option<&Path>) -> Result<()> {
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
        "vfat" | "fat16" | "fat32" => (FileSystem::Fat32, "defaults,nofail"),
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

pub fn get_recommend_swap_size(mem: u64) -> Result<f64> {
    // 1073741824 is 1 * 1024 * 1024 * 1024 (1GiB => 1iB)
    let swap_size = match mem {
        x @ ..=1073741824 => (x * 2) as f64,
        x @ 1073741825.. => {
            let x = x as f64;
            x + x.sqrt().round()
        }
    };

    Ok(swap_size)
}

pub fn is_enable_hibernation(custom_size: f64) -> Result<bool> {
    // Get men (iB)
    let mem = sysinfo::System::new_all().total_memory();
    let recommand_size = get_recommend_swap_size(mem)?;
    let no_hibernation_size = recommand_size - mem as f64;
    if custom_size >= no_hibernation_size && custom_size < recommand_size {
        return Ok(false);
    } else if custom_size >= recommand_size {
        return Ok(true);
    }

    // Round back to GiB for display message.
    Err(anyhow!("The specified swapfile size is too small, AOSC OS recommends at least {} GiB for your device.", (recommand_size / 1024.0 / 1024.0 / 1024.0).round()))
}

#[cfg(debug_assertions)]
pub fn auto_create_partitions(dev: &Path) -> Result<Partition> {
    let mut device = libparted::Device::new(dev)?;
    let device = &mut device as *mut Device;
    let device = unsafe { &mut (*device) };
    let efi_size = 512 * 1024 * 1024;
    let partition_table_end_size = 1024 * 1024;
    let is_efi = is_efi_booted();

    let length = device.length();
    let sector_size = device.sector_size();

    let size = length * sector_size;

    if get_partition_table_type(Some(dev))
        .map(|x| x == "msdos")
        .unwrap_or(false)
        && size > 512 * (2_u64.pow(31) - 1)
    {
        bail!(
            r#"AOSC OS Installer has detected that you are trying to create a disk partition larger than 2TiB in the MBR partition table.
If you want to do this, change your computer's boot mode to UEFI mode."#
        );
    }

    let disk = libparted::Disk::new(&mut *device)?;
    let mut nums = vec![];

    for i in disk.parts() {
        let num = i.num();
        if num > 0 {
            nums.push(num as u32);
        }
    }

    let mut device = libparted::Device::new(dev)?;
    let device = &mut device as *mut Device;
    let device = unsafe { &mut (*device) };
    let mut disk = libparted::Disk::new(&mut *device)?;

    for i in nums {
        disk.remove_partition_by_number(i)?;
    }

    commit(&mut disk)?;

    let mut device = libparted::Device::new(dev)?;
    let device = &mut device as *mut Device;
    let device = unsafe { &mut (*device) };

    let mut disk = Disk::new_fresh(
        &mut *device,
        if !is_efi {
            DiskType::get("msdos").unwrap()
        } else {
            DiskType::get("gpt").unwrap()
        },
    )?;

    disk.commit_to_dev()?;

    let mut device = libparted::Device::new(dev)?;
    let device = &mut device as *mut Device;
    let device = unsafe { &mut (*device) };

    let system_end_sector = if is_efi {
        length - (efi_size + partition_table_end_size) / sector_size
    } else {
        length - partition_table_end_size / sector_size
    };

    let mut flags = vec![];

    if !is_efi {
        flags.push(PedPartitionFlag::PED_PARTITION_BOOT);
    }

    let system = &PartitionCreate {
        path: dev.to_path_buf(),
        start_sector: 2048,
        end_sector: system_end_sector,
        format: true,
        file_system: Some(FileSystem::Ext4),
        kind: PartitionType::Primary,
        flags,
        label: None,
    };

    create_partition(device, system)?;

    let p = Partition {
        path: Some(PathBuf::from("/dev/loop20p1")),
        parent_path: Some(dev.to_path_buf()),
        fs_type: Some("ext4".to_string()),
        size: system_end_sector * device.sector_size(),
    };

    format_partition(&p)?;

    if is_efi {
        let start_sector = length - (partition_table_end_size + efi_size) / sector_size + 1;
        let efi = &PartitionCreate {
            path: dev.to_path_buf(),
            start_sector,
            end_sector: length - partition_table_end_size / sector_size,
            format: true,
            file_system: Some(FileSystem::Fat32),
            kind: PartitionType::Primary,
            flags: vec![
                PedPartitionFlag::PED_PARTITION_BOOT,
                PedPartitionFlag::PED_PARTITION_ESP,
            ],
            label: None,
        };

        create_partition(device, efi)?;

        let p = Partition {
            path: Some(PathBuf::from("/dev/loop20p2")),
            parent_path: Some(dev.to_path_buf()),
            fs_type: Some("vfat".to_string()),
            size: 512 * 1024_u64.pow(2),
        };

        format_partition(&p)?;
    }

    device.sync()?;

    Ok(p)
}

#[cfg(not(debug_assertions))]
pub fn auto_create_partitions(dev: &Path) -> Result<Partition> {
    let mut device = libparted::Device::new(dev)?;
    let device = &mut device as *mut Device;
    let device = unsafe { &mut (*device) };

    let is_efi = is_efi_booted();
    let sector_size = device.sector_size();

    let size = device.length() * sector_size;

    if get_partition_table_type(Some(dev))
        .map(|x| x == "msdos")
        .unwrap_or(false)
        && size > 512 * (2_u64.pow(31) - 1)
    {
        bail!(
            r#"AOSC OS Installer has detected that you are trying to create a disk partition larger than 2TiB in the MBR partition table.
If you want to do this, change your computer's boot mode to UEFI mode."#
        );
    }

    if let Ok(disk) = libparted::Disk::new(&mut *device) {
        info!("Disk already exists, open disk and reemove existing partitions");
        let mut nums = vec![];

        for i in disk.parts() {
            let num = i.num();
            if num > 0 {
                nums.push(num as u32);
            }
        }

        let mut device = libparted::Device::new(dev)?;
        let device = &mut device as *mut Device;
        let device = unsafe { &mut (*device) };
        let mut disk = libparted::Disk::new(&mut *device)?;

        for i in nums {
            disk.remove_partition_by_number(i)?;
        }

        commit(&mut disk)?;
    } else {
        info!("Disk does not exists, creating new ...");
    }

    let mut device = libparted::Device::new(dev)?;
    let device = &mut device as *mut Device;
    let device = unsafe { &mut (*device) };

    let mut disk = Disk::new_fresh(
        &mut *device,
        if !is_efi {
            DiskType::get("msdos").unwrap()
        } else {
            DiskType::get("gpt").unwrap()
        },
    )?;

    commit(&mut disk)?;

    let mut device = libparted::Device::new(dev)?;
    let device = &mut device as *mut Device;
    let mut device = unsafe { &mut (*device) };

    if is_efi {
        let efi = &PartitionCreate {
            path: dev.to_path_buf(),
            start_sector: 2048,
            end_sector: 2048 + (512 * 1024 * 1024 / device.sector_size()),
            format: true,
            file_system: Some(FileSystem::Fat32),
            kind: PartitionType::Primary,
            flags: vec![
                PedPartitionFlag::PED_PARTITION_BOOT,
                PedPartitionFlag::PED_PARTITION_ESP,
            ],
            label: None,
        };

        info!("10");
        create_partition(&mut device, efi)?;
    }

    let start_sector = if is_efi {
        2048 + (512 * 1024 * 1024 / sector_size) + 1
    } else {
        2048 + 1
    };

    let mut flags = vec![];

    if !is_efi {
        flags.push(PedPartitionFlag::PED_PARTITION_BOOT);
    }

    let length = device.length();

    let system = &PartitionCreate {
        path: dev.to_path_buf(),
        start_sector,
        end_sector: device.length() - 1 * 1024 * 1024 / sector_size,
        format: true,
        file_system: Some(FileSystem::Ext4),
        kind: PartitionType::Primary,
        flags,
        label: None,
    };
    info!("11");
    create_partition(&mut device, system)?;
    info!("12");
    let disk = libparted::Disk::new(&mut device)?;
    let mut last = None;
    for p in disk.parts() {
        if let Some(path) = p.get_path() {
            last = Some(path.to_path_buf());
        }
    }

    if is_efi {
        let part_efi = disk
            .get_partition_by_sector(2048)
            .ok_or_else(|| anyhow!("Could not find partition by sector: 2048"))?;

        let geom_length = part_efi.geom_length();
        let part_length = if geom_length < 0 {
            0
        } else {
            geom_length as u64
        };

        let p = Partition {
            path: part_efi.get_path().map(|x| x.to_path_buf()),
            parent_path: Some(dev.to_path_buf()),
            fs_type: Some("vfat".to_string()),
            size: part_length * sector_size,
        };

        format_partition(&p)?;
    }

    let p = last.ok_or_else(|| anyhow!("Cannot create partition"))?;

    let p = Partition {
        path: Some(p),
        parent_path: Some(dev.to_path_buf()),
        fs_type: Some("ext4".to_owned()),
        size: (length - start_sector) * sector_size,
    };

    format_partition(&p)?;

    Ok(p)
}

/// Defines a new partition to be created on the file system.
#[derive(Debug, Clone, PartialEq)]
pub struct PartitionCreate {
    /// The location of the disk in the system.
    pub path: PathBuf,
    /// The start sector that the partition will have.
    pub start_sector: u64,
    /// The end sector that the partition will have.
    pub end_sector: u64,
    /// Whether the filesystem should be formatted.
    pub format: bool,
    /// The format that the file system should be formatted to.
    pub file_system: Option<FileSystem>,
    /// Whether the partition should be primary or logical.
    pub kind: PartitionType,
    /// Flags which should be set on the partition.
    pub flags: Vec<PedPartitionFlag>,
    /// Defines the label to apply
    pub label: Option<String>,
}

impl BlockDeviceExt for PartitionCreate {
    fn get_device_path(&self) -> &Path {
        &self.path
    }

    fn get_mount_point(&self) -> Option<&Path> {
        None
    }
}

impl PartitionExt for PartitionCreate {
    fn get_file_system(&self) -> Option<FileSystem> {
        self.file_system
    }

    fn get_sector_end(&self) -> u64 {
        self.end_sector
    }

    fn get_sector_start(&self) -> u64 {
        self.start_sector
    }

    fn get_partition_flags(&self) -> &[PedPartitionFlag] {
        &self.flags
    }

    fn get_partition_label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    fn get_partition_type(&self) -> PartitionType {
        self.kind
    }
}

/// Creates a new partition on the device using the info in the `partition` parameter.
/// The partition table should reflect the changes before this function exits.
pub fn create_partition<P>(device: &mut Device, partition: &P) -> io::Result<()>
where
    P: PartitionExt,
{
    // Create a new geometry from the start sector and length of the new partition.
    let length = partition.get_sector_end() - partition.get_sector_start();
    let geometry = Geometry::new(device, partition.get_sector_start() as i64, length as i64)
        .map_err(|why| io::Error::new(why.kind(), format!("failed to create geometry: {}", why)))?;

    // Convert our internal partition type enum into libparted's variant.
    let part_type = match partition.get_partition_type() {
        PartitionType::Primary => PedPartitionType::PED_PARTITION_NORMAL,
        PartitionType::Logical => PedPartitionType::PED_PARTITION_LOGICAL,
        PartitionType::Extended => PedPartitionType::PED_PARTITION_EXTENDED,
    };

    // Open the disk, create the new partition, and add it to the disk.
    let (start, end) = (geometry.start(), geometry.start() + geometry.length());

    info!(
        "creating new partition with {} sectors: {} - {}",
        length, start, end
    );

    let fs_type = partition
        .get_file_system()
        .and_then(|fs| FileSystemType::get(fs.into()));

    {
        let mut disk = open_disk(device)?;
        let mut part =
            PedPartition::new(&disk, part_type, fs_type.as_ref(), start, end).map_err(|why| {
                io::Error::new(
                    why.kind(),
                    format!(
                        "failed to create new partition: {}: {}",
                        partition.get_device_path().display(),
                        why
                    ),
                )
            })?;

        for &flag in partition.get_partition_flags() {
            if part.is_flag_available(flag) && part.set_flag(flag, true).is_err() {
                error!("unable to set {:?}", flag);
            }
        }

        if let Some(label) = partition.get_partition_label() {
            if part.set_name(label).is_err() {
                error!("unable to set partition name: {}", label);
            }
        }

        // Add the partition, and commit the changes to the disk.
        let constraint = geometry.exact().expect("exact constraint not found");
        disk.add_partition(&mut part, &constraint).map_err(|why| {
            io::Error::new(
                why.kind(),
                format!(
                    "failed to create new partition: {}: {}",
                    partition.get_device_path().display(),
                    why
                ),
            )
        })?;

        // Attempt to write the new partition to the disk.
        info!(
            "committing new partition ({}:{}) on {}",
            start,
            end,
            partition.get_device_path().display()
        );

        commit(&mut disk)?;
    }

    device.sync()?;

    Ok(())
}

/// Opens a `libparted::Disk` from a `libparted::Device`.
pub fn open_disk<'a>(device: &'a mut Device) -> io::Result<Disk<'a>> {
    info!("opening disk at {}", device.path().display());
    let device = device as *mut Device;
    unsafe {
        match Disk::new(&mut *device) {
            Ok(disk) => Ok(disk),
            Err(_) => {
                info!("unable to open disk; creating new table on it");
                Disk::new_fresh(
                    &mut *device,
                    if !is_efi_booted() {
                        DiskType::get("msdos").unwrap()
                    } else {
                        DiskType::get("gpt").unwrap()
                    },
                )
            }
            .map_err(|why| {
                io::Error::new(
                    why.kind(),
                    format!(
                        "failed to create new partition table on {:?}: {}",
                        (*device).path(),
                        why
                    ),
                )
            }),
        }
    }
}

/// Attempts to commit changes to the disk, return a `DiskError` on failure.
pub fn commit(disk: &mut Disk) -> io::Result<()> {
    info!("committing changes to {}", unsafe {
        disk.get_device().path().display()
    });

    disk.commit().map_err(|why| {
        io::Error::new(
            why.kind(),
            format!(
                "failed to commit libparted changes to {:?}: {}",
                unsafe { disk.get_device() }.path(),
                why
            ),
        )
    })
}

#[test]
fn test_fs_recommendation() {
    assert_eq!(get_recommended_fs_type("btrfs"), "btrfs");
    assert_eq!(get_recommended_fs_type("ext2"), "ext4");
}

#[test]
fn test_recommend_swap_size() {
    let recommand_size = get_recommend_swap_size(16 * 1024 * 1024 * 1024).unwrap();
    assert_eq!(recommand_size, 17180000256.0);

    let recommand_size = get_recommend_swap_size((0.5 * 1024.0 * 1024.0 * 1024.0) as u64).unwrap();
    assert_eq!(recommand_size, 1073741824.0);
}
