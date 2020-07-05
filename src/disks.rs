use failure::{format_err, Error};
use libparted;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

const EFI_DETECT_PATH: &str = "/sys/firmware/efi";
const ALLOWED_FS_TYPE: &[&str] = &["ext4", "xfs", "btrfs", "f2fs"];
const DEFAULT_FS_TYPE: &str = "ext4";

#[derive(Debug, Clone)]
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

pub fn format_partition(partition: &Partition) -> Result<(), Error> {
    let default_fs = DEFAULT_FS_TYPE.to_owned();
    let fs_type = partition.fs_type.as_ref().unwrap_or(&default_fs);
    let mut command = Command::new(format!("mkfs.{}", fs_type));
    let cmd;
    let output;
    if fs_type == "ext4" {
        cmd = command.arg("-Fq");
    } else {
        cmd = command.arg("-f");
    }
    output = cmd.arg(partition.path.as_ref().ok_or(format_err!("Path not found"))?).output()?;
    if !output.status.success() {
        return Err(format_err!(
            "Failed to create filesystem: \n{}\n{}",
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        ));
    }

    Ok(())
}

pub fn fill_fs_type(part: &Partition) -> Partition {
    let mut new_part = part.clone();
    let new_fs_type: String;
    if let Some(fs_type) = new_part.fs_type.clone() {
        new_fs_type = get_recommended_fs_type(&fs_type).to_string();
    } else {
        new_fs_type = DEFAULT_FS_TYPE.to_string();
    }
    new_part.fs_type = Some(new_fs_type);

    new_part
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
                let part_length: u64;
                if geom_length < 0 {
                    part_length = 0;
                } else {
                    part_length = geom_length as u64;
                }
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

#[test]
fn test_fs_recommendation() {
    assert_eq!(get_recommended_fs_type("btrfs"), "btrfs");
    assert_eq!(get_recommended_fs_type("ext2"), "ext4");
}
