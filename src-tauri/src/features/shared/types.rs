use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StorageConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Error,
}

impl Default for StorageConnectionStatus {
    fn default() -> Self {
        StorageConnectionStatus::Disconnected
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub peer_id: Option<String>,
    pub version: Option<String>,
    pub repo_path: Option<String>,
    pub connected_peers: u32,
    pub max_peers: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageInfo {
    pub used_bytes: u64,
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub block_count: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadResultResponse {
    pub cid: String,
    pub size: usize,
    pub duration_ms: u64,
    pub verified: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadResultResponse {
    pub cid: String,
    pub size: usize,
    pub duration_ms: u64,
    pub verified: bool,
    pub filepath: Option<String>,
}
