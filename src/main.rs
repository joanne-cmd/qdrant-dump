mod client;
mod dump;
mod models;
mod utils;

use clap::Parser;
use std::path::PathBuf;
use crate::{client::QdrantClient, dump::dump_collection, utils::create_dir_with_timestamp};

/// CLI for dumping Qdrant collections as snapshots.
#[derive(Parser, Debug)]
#[command(name = "qdrant-dump")]
#[command(about = "Backup collections or full snapshots from Qdrant easily")]
struct Args {
    /// Qdrant server URL (e.g., https://your-qdrant-host.com)
    #[arg(short, long)]
    url: String,

    /// API key for Qdrant (if needed)
    #[arg(short, long, default_value = "")]
    api_key: String,

    /// Collection to back up, or "all" for all collections
    #[arg(short, long, default_value = "all")]
    collection: String,

    /// Directory where backups will be saved
    #[arg(short, long, default_value = "./backups")]
    out: PathBuf,

    /// Add timestamp to backup folder
    #[arg(short, long)]
    timestamp: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let backup_dir = create_dir_with_timestamp(&args.out, args.timestamp)?;

    println!("📦 Starting Qdrant backup...");
    println!("🔗 Server: {}", args.url);
    println!("📁 Output: {}", backup_dir.display());
    println!();

    let client = QdrantClient::new(&args.url, &args.api_key);

    // If collection == "all", dump every collection
    if args.collection == "all" {
        let collections = client.get_collections().await?;
        println!("Found {} collections.\n", collections.len());

        for col in collections {
            dump_collection(&client, &col.name, &backup_dir).await?;
        }
    } else {
        dump_collection(&client, &args.collection, &backup_dir).await?;
    }

    println!("\n✅ Backup complete!");
    Ok(())
}

