use failure::{format_err, Error};
use hex;
use nix::mount;
use nix::unistd::{chroot, sync};
use nix::sys::reboot::{reboot, RebootMode};
use sha2::{Digest, Sha256};
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;
use tar;
use xz2;

use crate::disks::Partition;

const BIND_MOUNTS: &[&str] = &["/dev", "/proc", "/sys", "/run/udev"];

pub fn extract_tar_xz<R: Read>(reader: R, path: &PathBuf) -> Result<(), Error> {
    let decompress = xz2::read::XzDecoder::new(reader);
    let mut tar_processor = tar::Archive::new(decompress);
    tar_processor.unpack(path)?;

    Ok(())
}

pub fn sha256sum<R: Read>(mut reader: R) -> Result<String, Error> {
    let mut hasher = Sha256::new();
    std::io::copy(&mut reader, &mut hasher)?;

    Ok(hex::encode(hasher.finalize()))
}

pub fn auto_mount_root_path(partition: &Partition) -> Result<PathBuf, Error> {
    let tmp_dir = TempDir::new()?;
    let tmp_path = tmp_dir.into_path();
    mount_root_path(partition, &tmp_path)?;

    Ok(tmp_path)
}

pub fn sync_and_reboot() -> Result<(), Error> {
    sync();
    reboot(RebootMode::RB_AUTOBOOT)?;

    Ok(())
}

pub fn mount_root_path(partition: &Partition, target: &PathBuf) -> Result<(), Error> {
    if partition.fs_type.is_none() || partition.path.is_none() {
        return Err(format_err!("Path not specified."));
    }
    let source = partition.path.as_ref();
    let mut fs_type = partition.fs_type.as_ref().unwrap().as_str();
    if fs_type.starts_with("fat") {
        fs_type = "vfat";
    }
    // FIXME: due to an issue in `nix` and `libc`, `MS_LAZYTIME` is not supported atm
    mount::mount(
        source,
        target,
        Some(fs_type),
        mount::MsFlags::empty(),
        None::<&str>,
    )?;

    Ok(())
}

pub fn umount_root_path(root: &PathBuf) -> Result<(), Error> {
    mount::umount2(root, mount::MntFlags::MNT_DETACH)?;

    Ok(())
}

pub fn get_root_distance(path: &PathBuf) -> Result<usize, Error> {
    let path = path.canonicalize()?;

    Ok(path.components().count() + 4)
}

pub fn escape_chroot(distance: usize) -> Result<(), Error> {
    let escape_path = "../".repeat(distance);
    chroot(escape_path.as_str())?;
    std::env::set_current_dir("/")?;  // reset cwd (on host)

    Ok(())
}

pub fn setup_bind_mounts(root: &PathBuf) -> Result<(), Error> {
    for mount in BIND_MOUNTS {
        let mut root = root.clone();
        root.push(&mount[1..]);
        std::fs::create_dir_all(root.clone())?;
        mount::mount(
            Some(*mount),
            &root,
            None::<&str>,
            mount::MsFlags::MS_BIND,
            None::<&str>,
        )?;
    }

    Ok(())
}

/// Remove bind mounts
/// Note: This function should be called outside of the chroot context
pub fn remove_bind_mounts(root: &PathBuf) -> Result<(), Error> {
    for mount in BIND_MOUNTS {
        let mut root = root.clone();
        root.push(&mount[1..]);
        mount::umount2(&root, mount::MntFlags::MNT_DETACH)?;
    }

    Ok(())
}

/// Setup bind mounts and chroot into the guest system
/// Warning: This will make the program trapped in the new root directory
pub fn dive_into_guest(root: &PathBuf) -> Result<(), Error> {
    setup_bind_mounts(root)?;
    chroot(root)?;
    std::env::set_current_dir("/")?;  // jump to the root directory after chroot

    Ok(())
}

/// Runs dracut
/// Must be used in a chroot context
pub fn execute_dracut() -> Result<(), Error> {
    let output = Command::new("sh")
        .arg("/var/ab/triggered/dracut")
        .output()?;
    if !output.status.success() {
        return Err(format_err!(
            "Failed to execute dracut: \n{}\n{}",
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        ));
    }

    Ok(())
}

/// Runs grub-install and grub-mkconfig
/// Must be used in a chroot context
pub fn execute_grub_install(mbr_dev: Option<&PathBuf>) -> Result<(), Error> {
    let mut command = Command::new("grub-install");
    let cmd;
    if let Some(mbr_dev) = mbr_dev {
        cmd = command.arg("--target=i386-pc").arg(mbr_dev);
    } else {
        cmd = command
            .arg("--target=x86_64-efi")
            .arg("--bootloader-id=AOSC OS")
            .arg("--efi-directory=/efi");
    }
    let process = cmd.output()?;
    if !process.status.success() {
        return Err(format_err!(
            "Failed to execute grub-install: {}",
            String::from_utf8_lossy(&process.stderr)
        ));
    }
    let process = Command::new("grub-mkconfig")
        .arg("-o")
        .arg("/boot/grub/grub.cfg")
        .output()?;
    if !process.status.success() {
        return Err(format_err!(
            "Failed to execute grub-mkconfig: {}",
            String::from_utf8_lossy(&process.stderr)
        ));
    }

    Ok(())
}

#[test]
fn test_path_strip() {
    for mount in BIND_MOUNTS {
        println!("{}", &mount[1..]);
    }
}
