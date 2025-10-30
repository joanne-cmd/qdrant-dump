use chrono::Local;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::Result;

pub fn create_dir_with_timestamp(base: &Path, timestamp: bool) -> Result<PathBuf> {
    let mut dir = base.to_path_buf();
    if timestamp {
        let ts = Local::now().format("%Y%m%d_%H%M%S");
        dir = dir.join(ts.to_string());
    }
    fs::create_dir_all(&dir)?;
    Ok(dir)
}
