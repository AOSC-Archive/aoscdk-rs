use libparted;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Partition {
    pub path: Option<PathBuf>,
    pub fs_type: Option<String>,
    pub size: u64,
}

pub fn list_partitions() -> Vec<Partition> {
    let mut partitions: Vec<Partition> = Vec::new();
    for mut device in libparted::Device::devices(true) {
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
                    size: sector_size * part_length,
                    fs_type,
                });
            }
        }
    }

    partitions
}
