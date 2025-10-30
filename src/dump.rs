use std::path::PathBuf;
use std::fs;
use indicatif::ProgressBar;
use crate::{client::QdrantClient, models::SnapshotResponse};
use anyhow::Result;

pub async fn dump_collection(client: &QdrantClient, name: &str, dir: &PathBuf) -> Result<()> {
    println!("📂 Dumping collection: {}", name);

    // Step 1: Create snapshot
    let pb = ProgressBar::new_spinner();
    pb.set_message("Creating snapshot...");
    let SnapshotResponse { result } = client.create_snapshot(name).await?;
    pb.finish_with_message("Snapshot created ✅");

    // Step 2: Download snapshot
    let pb = ProgressBar::new_spinner();
    pb.set_message("Downloading...");
    let bytes = client.download_snapshot(name, &result.name).await?;
    pb.finish_with_message("Downloaded ✅");

    // Step 3: Save snapshot
    let file_path = dir.join(format!("{}_{}", name, result.name));
    fs::write(&file_path, &bytes)?;
    println!("💾 Saved: {}", file_path.display());

    Ok(())
}
