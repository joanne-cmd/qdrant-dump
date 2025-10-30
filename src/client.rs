use reqwest::Client;
use crate::models::{CollectionsResponse, SnapshotResponse};
use anyhow::Result;

pub struct QdrantClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl QdrantClient {
    pub fn new(base_url: &str, api_key: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key: api_key.to_string(),
        }
    }

    fn auth(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if !self.api_key.is_empty() {
            req.header("api-key", &self.api_key)
        } else {
            req
        }
    }

    pub async fn get_collections(&self) -> Result<Vec<crate::models::CollectionInfo>> {
        let url = format!("{}/collections", self.base_url);
        let res = self.auth(self.client.get(&url)).send().await?;
        let res = res.error_for_status()?;
        let parsed: CollectionsResponse = res.json().await?;
        Ok(parsed.result.collections)
    }

    pub async fn create_snapshot(&self, collection: &str) -> Result<SnapshotResponse> {
        let encoded = urlencoding::encode(collection);
        let url = format!("{}/collections/{}/snapshots", self.base_url, encoded);
        let res = self.auth(self.client.post(&url)).send().await?;
        let res = res.error_for_status()?;
        Ok(res.json().await?)
    }

    pub async fn download_snapshot(&self, collection: &str, name: &str) -> Result<bytes::Bytes> {
        let encoded_collection = urlencoding::encode(collection);
        let encoded_name = urlencoding::encode(name);
        let url = format!("{}/collections/{}/snapshots/{}", self.base_url, encoded_collection, encoded_name);
        let res = self.auth(self.client.get(&url)).send().await?;
        let res = res.error_for_status()?;
        Ok(res.bytes().await?)
    }
}
