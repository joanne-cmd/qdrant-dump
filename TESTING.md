# Testing Guide for qdrant-dump

## Overview
This guide provides practical testing strategies for the `qdrant-dump` tool, similar to how mongodump is tested.

## Prerequisites

### 1. Set up a Local Qdrant Instance

**Option A: Docker (Recommended)**

Run in detached mode (runs in background, frees up terminal):
```bash
docker run -d -p 6333:6333 --name qdrant qdrant/qdrant
```

Or run in foreground (for seeing logs, but blocks terminal):
```bash
docker run -p 6333:6333 qdrant/qdrant
```

**Tip:** If you run in detached mode (`-d`), you can view logs later with:
```bash
docker logs -f qdrant
```

To stop Qdrant later:
```bash
docker stop qdrant
docker rm qdrant  # Optional: remove container after stopping
```

**Option B: Install Qdrant locally**
Follow instructions at: https://qdrant.tech/documentation/quick-start/

### 2. Build the Tool
```bash
cargo build --release
```

### 3. Create Test Collections (one command)

Use the provided setup script (no Python required):
```bash
chmod +x quick_test_setup.sh
./quick_test_setup.sh
```

This will start Qdrant (if not running) and create three collections:
- `product_embeddings` (128-dim)
- `user_profiles` (256-dim)
- `document_search` (384-dim)

## Test Scenarios

### Basic Functionality Tests

#### Test 1: Dump Single Collection
```bash
# Dump the product_embeddings collection (created in setup)
./target/release/qdrant-dump \
  --url http://localhost:6333 \
  --collection product_embeddings \
  --out ./backups \
  --timestamp

# Verify:
# - Backup directory created with timestamp
# - Snapshot file exists: product_embeddings_<snapshot-name>
# - File size > 0 (check with: ls -lh ./backups/*/product_embeddings_*)
# - File has correct naming pattern
ls -lh ./backups/*/
```

#### Test 2: Dump All Collections
```bash
# Dump all collections (product_embeddings, user_profiles, document_search)
./target/release/qdrant-dump \
  --url http://localhost:6333 \
  --collection all \
  --out ./backups

# Verify:
# - All 3 collections dumped
# - Files created:
#   * product_embeddings_<snapshot-name>
#   * user_profiles_<snapshot-name>
#   * document_search_<snapshot-name>
# - No duplicate files
# - Progress messages shown for each
ls -lh ./backups/
```

#### Test 3: Empty Collection
```bash
# Create an empty collection first
curl -X PUT "http://localhost:6333/collections/empty_test" \
  -H "Content-Type: application/json" \
  -d '{"vectors": {"size": 128, "distance": "Cosine"}}'

# Dump the empty collection
./target/release/qdrant-dump \
  --url http://localhost:6333 \
  --collection empty_test \
  --out ./backups

# Verify snapshot is created (even if empty)
ls -lh ./backups/empty_test_*
```

### Error Handling Tests

#### Test 4: Non-existent Collection
```bash
./target/release/qdrant-dump \
  --url http://localhost:6333 \
  --collection nonexistent_collection \
  --out ./test_backups

# Expected: Clear error message (404)
```

#### Test 5: Invalid URL
```bash
./target/release/qdrant-dump \
  --url http://invalid-host:6333 \
  --collection test \
  --out ./test_backups

# Expected: Connection error
```

#### Test 6: Wrong API Key (if authentication enabled)
```bash
./target/release/qdrant-dump \
  --url http://localhost:6333 \
  --api-key wrong-key \
  --collection test \
  --out ./test_backups

# Expected: Authentication error (401/403)
```

#### Test 7: Collection Name with Special Characters
```bash
# Create a collection with special characters (realistic scenario)
curl -X PUT "http://localhost:6333/collections/prod-data_v2.1" \
  -H "Content-Type: application/json" \
  -d '{"vectors": {"size": 128, "distance": "Cosine"}}'

./target/release/qdrant-dump \
  --url http://localhost:6333 \
  --collection "prod-data_v2.1" \
  --out ./backups

# Expected: Should handle URL encoding properly
ls -lh ./backups/prod-data_v2.1_*
```

### Edge Cases

#### Test 8: Performance Sanity Check
```bash
# Measure dump time for existing collections
time ./target/release/qdrant-dump \
  --url http://localhost:6333 \
  --collection product_embeddings \
  --out ./backups

time ./target/release/qdrant-dump \
  --url http://localhost:6333 \
  --collection all \
  --out ./backups

# Verify file sizes look reasonable
ls -lh ./backups/*/*.snapshot
```

#### Test 9: Concurrent Snapshots
```bash
# Run dumps for different collections simultaneously
./target/release/qdrant-dump --url http://localhost:6333 --collection product_embeddings --out ./test1 &
./target/release/qdrant-dump --url http://localhost:6333 --collection user_profiles --out ./test2 &
wait

# Verify both complete successfully
ls -lh ./test1/ ./test2/
```

#### Test 10: Network Interruption Simulation
```bash
# 1. Start dump
# 2. Kill Qdrant instance mid-download
# 3. Verify graceful error handling
```

### Integration Tests

#### Test 11: Verify Snapshot Format
```bash
# Dump a collection
./target/release/qdrant-dump \
  --url http://localhost:6333 \
  --collection product_embeddings \
  --out ./backups

# Verify snapshot format is valid
SNAPSHOT_FILE=$(ls ./backups/*/product_embeddings_* | head -1)
du -h "$SNAPSHOT_FILE"  # Check file size

# Verify it's not empty and appears to be binary data (Qdrant snapshots are binary)
if [ -s "$SNAPSHOT_FILE" ]; then
    echo "✅ Snapshot file is not empty"
    head -c 64 "$SNAPSHOT_FILE" | hexdump -C | head -3  # preview bytes
fi
```

#### Test 12: Multiple Runs (Timestamp vs No Timestamp)
```bash
# Run 1: with --timestamp (creates timestamped subdirectory)
./target/release/qdrant-dump \
  --url http://localhost:6333 \
  --collection product_embeddings \
  --out ./backups \
  --timestamp

# Check timestamped directory was created
ls -d ./backups/2024* 

# Run 2: without --timestamp (uses base directory directly)
./target/release/qdrant-dump \
  --url http://localhost:6333 \
  --collection product_embeddings \
  --out ./backups

# Verify: file is in ./backups/ directory (not timestamped subdir)
ls -lh ./backups/product_embeddings_*
```

## Automated Testing Script

Create a test script to automate the above:

```bash
#!/bin/bash
# test-qdrant-dump.sh

set -e

QDRANT_URL="${QDRANT_URL:-http://localhost:6333}"
TEST_OUT_DIR="./test_backups_$(date +%s)"

echo "🧪 Testing qdrant-dump..."

# Test 1: Help/Version
echo "Test 1: Help"
./target/release/qdrant-dump --help || exit 1

# Test 2: List collections (if we add that feature)
echo "Test 2: Connect to Qdrant"
curl -s "$QDRANT_URL/collections" || {
    echo "❌ Qdrant not running at $QDRANT_URL"
    exit 1
}

# Test 3: Dump all (will test with whatever collections exist)
echo "Test 3: Dump all collections"
./target/release/qdrant-dump \
    --url "$QDRANT_URL" \
    --collection all \
    --out "$TEST_OUT_DIR" || exit 1

# Verify output exists and has files
if [ ! -d "$TEST_OUT_DIR" ]; then
    echo "❌ Output directory not created"
    exit 1
fi

FILE_COUNT=$(find "$TEST_OUT_DIR" -type f | wc -l)
if [ "$FILE_COUNT" -eq 0 ]; then
    echo "❌ No backup files created"
    exit 1
fi

echo "✅ Created $FILE_COUNT backup file(s)"
echo "✅ Basic tests passed!"
```

## Manual Test Checklist

- [ ] Dump single collection succeeds
- [ ] Dump all collections succeeds  
- [ ] Non-existent collection shows clear error
- [ ] Invalid URL shows clear error
- [ ] Timestamp flag creates timestamped folder
- [ ] No timestamp flag uses base directory
- [ ] Special characters in collection name work
- [ ] Progress indicators display correctly
- [ ] API key authentication works (if applicable)
- [ ] Large files download successfully
- [ ] Output files are valid (non-zero size)

## Performance Benchmarks

For large-scale testing:

```bash
# Measure dump time for different collection sizes
echo "Testing product_embeddings (100 points)"
time ./target/release/qdrant-dump \
  --url http://localhost:6333 \
  --collection product_embeddings \
  --out ./backups

# Check memory usage
/usr/bin/time -v ./target/release/qdrant-dump \
  --url http://localhost:6333 \
  --collection user_profiles \
  --out ./backups

# Compare with large collection (if created)
if curl -s http://localhost:6333/collections/large_dataset > /dev/null 2>&1; then
    echo "Testing large_dataset (10K points)"
    time ./target/release/qdrant-dump \
      --url http://localhost:6333 \
      --collection large_dataset \
      --out ./backups
fi
```

## Recommended Testing Tools

1. **MockQdrant**: Use a mock HTTP server for unit tests
2. **Testcontainers**: Docker-based integration tests
3. **Fixture Data**: Create standardized test collections

## Future Test Cases (When Features Added)

- [ ] Restore functionality (qdrant-restore)
- [ ] Snapshot cleanup option
- [ ] Resume interrupted downloads
- [ ] Compression options
- [ ] Incremental backups
- [ ] Backup verification (checksums)

