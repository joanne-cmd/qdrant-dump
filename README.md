# qdrant-dump

A command-line tool for backing up Qdrant vector database collections, inspired by `mongodump`. Create and download snapshots of your Qdrant collections with ease.

## 🚀 Features

- **Single or All Collections**: Backup a specific collection or all collections at once
- **Automatic Snapshots**: Creates Qdrant snapshots and downloads them automatically
- **Progress Indicators**: Real-time progress feedback during backup operations
- **Timestamped Backups**: Optional timestamp-based directory organization
- **Secure Authentication**: Supports API key authentication
- **Robust Error Handling**: Clear error messages for common issues
- **URL Encoding**: Handles collection names with special characters safely
- **Zero Dependencies**: Self-contained binary (after compilation)

## 📋 Requirements

- Rust 1.70+ (for building from source)
- Qdrant server (local or remote)
- Network access to your Qdrant instance

## 🛠️ Installation

### Build from Source

```bash
git clone <repository-url>
cd qdrant-dump
cargo build --release
```

The binary will be available at `./target/release/qdrant-dump`.

### Install Globally (Optional)

Install and run from anywhere:
```bash
cargo install --path .
export PATH="$HOME/.cargo/bin:$PATH"  
qdrant-dump --help                     
```

## 📖 Usage

### Basic Examples

**Backup a single collection:**
```bash
qdrant-dump \
  --url http://localhost:6333 \
  --collection product_embeddings \
  --out ./backups
```

**Backup all collections with timestamp:**
```bash
qdrant-dump \
  --url http://localhost:6333 \
  --collection all \
  --out ./backups \
  --timestamp
```

**Backup from remote Qdrant Cloud with API key:**
```bash
qdrant-dump \
  --url https://your-cluster.qdrant.io \
  --api-key your-api-key \
  --collection all \
  --out ./backups \
  --timestamp
```

### Command Line Options

```
Usage: qdrant-dump [OPTIONS] --url <URL>

Options:
  -u, --url <URL>              Qdrant server URL (e.g., https://your-qdrant-host.com)
  -k, --api-key <API_KEY>       API key for Qdrant (if needed) [default: ]
  -c, --collection <COLLECTION>
                                Collection to back up, or "all" for all collections
                                [default: all]
  -o, --out <OUT>               Directory where backups will be saved [default: ./backups]
  -t, --timestamp               Add timestamp to backup folder
  -h, --help                    Print help
```

### How It Works

1. **Connects** to your Qdrant instance
2. **Creates** a snapshot for the specified collection(s)
3. **Downloads** the snapshot file(s) to your local filesystem
4. **Saves** them with descriptive filenames: `{collection_name}_{snapshot_name}.snapshot`

The snapshot files are native Qdrant snapshot format and can be restored using Qdrant's restore functionality.

## 🔧 Testing

### Quick Test Setup

1. **Start Qdrant locally:**
```bash
docker run -d -p 6333:6333 --name qdrant qdrant/qdrant
```

2. **Create test collections :**
```bash
chmod +x quick_test_setup.sh
./quick_test_setup.sh
```

3. **Run a test backup:**
```bash
cargo build --release
./target/release/qdrant-dump \
  --url http://localhost:6333 \
  --collection all \
  --out ./backups \
  --timestamp
```

See [TESTING.md](./TESTING.md) for comprehensive testing scenarios.

## 📁 Output Format

Backups are saved as `.snapshot` files with the naming pattern:
```
{collection_name}_{snapshot_id}.snapshot
```

Example:
```
product_embeddings_product_embeddings-6070297544516242-2025-10-29-11-58-35.snapshot
```

When using `--timestamp`, files are organized in timestamped directories:
```
backups/
  └── 20251029_125835/
      ├── product_embeddings_*.snapshot
      ├── user_profiles_*.snapshot
      └── document_search_*.snapshot
```

## 🔄 Restoring Backups

To restore a snapshot, you can use Qdrant's REST API:

```bash
curl -X PUT "http://localhost:6333/collections/{collection_name}/snapshots/upload" \
  -H "Content-Type: multipart/form-data" \
  -F "snapshot=@/path/to/backup.snapshot"
```

Or use Qdrant's web UI at `http://localhost:6333/dashboard`.

## 🐛 Troubleshooting

### Connection Refused
- Ensure Qdrant is running: `docker ps | grep qdrant`
- Check the URL: `http://localhost:6333` (default) or your remote URL
- Verify network connectivity

### 404 Not Found (Collection)
- Collection doesn't exist - verify with: `curl http://localhost:6333/collections`
- Check spelling of collection name
- Ensure you have read permissions

### Authentication Errors (401/403)
- Verify your API key is correct
- Check if your Qdrant instance requires authentication
- Ensure the API key has proper permissions

### Invalid URL
- URLs must include protocol: `http://` or `https://`
- For localhost: `http://localhost:6333`
- Trailing slashes are automatically handled


## 🔮 Future Enhancements

Potential features for future releases:

- [ ] Restore functionality (`qdrant-restore`)
- [ ] Snapshot cleanup option (delete after download)
- [ ] Compression support (gzip, zstd)
- [ ] Incremental backups
- [ ] Backup verification (checksums)
- [ ] Resume interrupted downloads
- [ ] Parallel collection dumping
- [ ] S3/cloud storage upload support
- [ ] Configuration file support
- [ ] Verbose logging mode


## 📚 Related Tools

- [Qdrant](https://qdrant.tech) - Vector similarity search engine
- [qdrant-client](https://github.com/qdrant/qdrant-client-rust) - Official Rust client

---


# qdrant-dump
