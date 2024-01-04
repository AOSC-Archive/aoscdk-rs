use anyhow::{anyhow, Result};
use log::setup_logger;
use once_cell::sync::OnceCell;
use std::{
    io::{Read, Write},
    path::{Path, PathBuf},
    str::FromStr,
};
use sysinfo::{Pid, System};

use clap::Parser;
use frontend::Args;

mod disks;
mod frontend;
mod install;
mod log;
mod network;
mod parser;

const LOCK: &str = "/run/lock/aoscdk.lock";

pub static LOG_FILE: OnceCell<PathBuf> = OnceCell::new();

fn main() {
    if let Err(e) = create_lock() {
        eprintln!("Installer failed to obtain the instance lock: {e}");
        std::process::exit(1);
    }

    if let Err(e) = execute() {
        eprintln!("{e}");
        remove_lock().ok();
        std::process::exit(1);
    }
    remove_lock().ok();
    std::process::exit(0);
}

fn execute() -> Result<()> {
    let args = std::env::args();
    if args.len() < 2 {
        LOG_FILE.get_or_try_init(|| setup_logger(false))?;
        frontend::tui_main();
    } else {
        let args = Args::parse();
        LOG_FILE.get_or_try_init(|| setup_logger(true))?;
        frontend::execute(args)?;
    }

    Ok(())
}

fn create_lock() -> Result<()> {
    let lock = Path::new(LOCK);
    if lock.is_file() {
        let mut lock_file = std::fs::File::open(lock)?;
        let mut old_pid = String::new();
        lock_file.read_to_string(&mut old_pid)?;

        let s = System::new_all();
        let old_pid = Pid::from_str(&old_pid)?;

        if s.process(old_pid).is_some() {
            return Err(anyhow!(
                "Another instance of Installer (pid: {}) is still running!",
                old_pid
            ));
        } else {
            remove_lock()?;
        }
    }
    let mut lock_file = std::fs::File::create(lock)?;
    let pid = std::process::id().to_string();
    lock_file.write_all(pid.as_bytes())?;

    Ok(())
}

fn remove_lock() -> Result<()> {
    std::fs::remove_file(LOCK)?;

    Ok(())
}
