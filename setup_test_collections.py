#!/usr/bin/env python3
"""
Setup script to create realistic test collections for qdrant-dump testing.
Run this before testing to ensure you have real collections to dump.
"""

from qdrant_client import QdrantClient
from qdrant_client.http import models
import random
import sys

def main():
    try:
        client = QdrantClient("localhost", port=6333)
        print("✅ Connected to Qdrant at localhost:6333\n")
    except Exception as e:
        print(f"❌ Failed to connect to Qdrant: {e}")
        print("Make sure Qdrant is running: docker run -p 6333:6333 qdrant/qdrant")
        sys.exit(1)

    # Create realistic collections
    collections = [
        {
            "name": "product_embeddings",
            "size": 128,
            "description": "E-commerce product search vectors"
        },
        {
            "name": "user_profiles",
            "size": 256,
            "description": "User embedding vectors for recommendations"
        },
        {
            "name": "document_search",
            "size": 384,
            "description": "Document embeddings for semantic search"
        }
    ]

    for col in collections:
        print(f"Creating collection: {col['name']}...")
        
        # Check if collection already exists
        try:
            info = client.get_collection(col["name"])
            print(f"  ⚠️  Collection already exists, deleting...")
            client.delete_collection(col["name"])
        except:
            pass
        
        # Create collection
        client.create_collection(
            collection_name=col["name"],
            vectors_config=models.VectorParams(
                size=col["size"],
                distance=models.Distance.COSINE
            )
        )
        
        # Add some sample points
        points = [
            models.PointStruct(
                id=i,
                vector=[random.random() for _ in range(col["size"])],
                payload={
                    "name": f"Item {i}",
                    "category": random.choice(["A", "B", "C"]),
                    "score": round(random.random() * 100, 2)
                }
            )
            for i in range(100)  # 100 points per collection
        ]
        client.upsert(collection_name=col["name"], points=points)
        print(f"  ✅ Created {col['name']} with 100 points\n")

    print("✅ All test collections created!")
    print("\nYou can now test qdrant-dump with:")
    print("  ./target/release/qdrant-dump --url http://localhost:6333 --collection all --out ./backups")

if __name__ == "__main__":
    main()

