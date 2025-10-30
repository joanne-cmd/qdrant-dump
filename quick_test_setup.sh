#!/bin/bash
# Quick setup script using only curl (no Python required)

set -e

echo "🔧 Starting Qdrant..."
docker run -d -p 6333:6333 --name qdrant qdrant/qdrant 2>/dev/null || \
  docker start qdrant 2>/dev/null || true

echo "⏳ Waiting for Qdrant to be ready..."
sleep 3

echo "📦 Creating test collections..."

# Create product_embeddings collection
echo "  Creating product_embeddings..."
curl -s -X PUT "http://localhost:6333/collections/product_embeddings" \
  -H "Content-Type: application/json" \
  -d '{
    "vectors": {
      "size": 128,
      "distance": "Cosine"
    }
  }' > /dev/null

# Create user_profiles collection
echo "  Creating user_profiles..."
curl -s -X PUT "http://localhost:6333/collections/user_profiles" \
  -H "Content-Type: application/json" \
  -d '{
    "vectors": {
      "size": 256,
      "distance": "Cosine"
    }
  }' > /dev/null

# Create document_search collection
echo "  Creating document_search..."
curl -s -X PUT "http://localhost:6333/collections/document_search" \
  -H "Content-Type: application/json" \
  -d '{
    "vectors": {
      "size": 384,
      "distance": "Cosine"
    }
  }' > /dev/null

echo "✅ Collections created!"
echo ""
echo "You can now test:"
echo "  ./target/release/qdrant-dump --url http://localhost:6333 --collection product_embeddings --out ./backups"
echo "  ./target/release/qdrant-dump --url http://localhost:6333 --collection all --out ./backups --timestamp"

