use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CollectionsResponse {
    pub result: CollectionsResult,
}

#[derive(Debug, Deserialize)]
pub struct CollectionsResult {
    pub collections: Vec<CollectionInfo>,
}

#[derive(Debug, Deserialize)]
pub struct CollectionInfo {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct SnapshotResponse {
    pub result: SnapshotResult,
}

#[derive(Debug, Deserialize)]
pub struct SnapshotResult {
    pub name: String,
}
