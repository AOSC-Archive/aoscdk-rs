use std::path::{Path, PathBuf};

use anyhow::Result;
use log::info;
use time::OffsetDateTime;

/// Log message to console and file
pub fn setup_logger(is_cli: bool) -> Result<PathBuf> {
    let now = OffsetDateTime::now_utc();
    let path = Path::new(&format!("/var/log/dklog-{}.log", now)).to_path_buf();

    fern::Dispatch::new()
        .format(move |out, message, record| {
            let now = OffsetDateTime::now_utc();
            out.finish(format_args!(
                "{}[{}][{}] {}",
                now,
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(fern::log_file(&path)?)
        .apply()?;

    info!(
        "Using AOSC Deplotkit {} mode",
        if is_cli { "CLI" } else { "TUI" }
    );

    Ok(path)
}
