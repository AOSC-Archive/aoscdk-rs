//! Communication and command backend

use std::path::PathBuf;
use serde::{Deserialize, Serialize};

pub use crate::disks::Partition;

pub struct DiskProbeResult {
    pub parts: Vec<Partition>,
    pub is_efi: bool,
}

pub struct ConfigSummary {
    pub root_part: Partition,
    pub boot_part: Partition,
    pub username: String,
    pub mirror: String,
    pub variant: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InstallConfig {
    pub variant_id: usize,
    pub root_partition_id: usize,
    pub boot_partition_id: usize,
    pub mirror_id: usize,
    pub locale_id: usize,
    pub rtc_is_utc: bool,
    pub timezone_id: usize,
    pub username: String,
    pub password: String,
    pub hostname: String,
}

pub enum ErrorKind {
    InternalError,
    NetworkError,
    ConfigError,
    Other,
}

pub struct Error {
    kind: ErrorKind,
    message: String,
    retryable: bool,
    ignorable: bool,
}

pub enum DKRequest {
    /// Request the tarball manifest from the server
    FetchManifest,
    /// load the state from the file
    LoadState(PathBuf),
    /// save the state to the file
    SaveState(PathBuf),
    /// set the installation config
    SetConfig(InstallConfig),
    /// start the install with specified parameters
    StartInstall,
    /// Fetch the locale list
    FetchLocales,
    /// probe disks
    ProbeDisk,
    /// reboot the system
    RebootSystem,
    /// request the work thread to quit
    Quit,
}

pub enum DKResponse {
    /// SetConfig response: (ConfigSummary)
    ConfigSummary(ConfigSummary),
    /// ProbeDisk response: (DiskProbeResult)
    ProbeDisk(DiskProbeResult),
    /// Progress notification: (progress [percentage, 0-100%, 101%], message)
    /// A progress with 101% means the progress could not be determined
    Progress(u64, String),
    /// Warning notification: (message)
    Warning(String),
    /// Finished notification
    Finished,
    /// Error message: (error)
    Error(Error),
    /// Request the client to shutdown
    Quit,
}
