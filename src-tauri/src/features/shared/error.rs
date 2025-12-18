use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageError {
    NodeCreation(String),
    NodeStart(String),
    NodeNotInitialized,
    NodeNotStarted,
    Upload(String),
    Download(String),
    FileNotFound(String),
    InvalidCid(String),
    Io(String),
    Configuration(String),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::NodeCreation(msg) => write!(f, "Failed to create node: {}", msg),
            StorageError::NodeStart(msg) => write!(f, "Failed to start node: {}", msg),
            StorageError::NodeNotInitialized => write!(f, "Node is not initialized"),
            StorageError::NodeNotStarted => write!(f, "Node is not started"),
            StorageError::Upload(msg) => write!(f, "Upload failed: {}", msg),
            StorageError::Download(msg) => write!(f, "Download failed: {}", msg),
            StorageError::FileNotFound(path) => write!(f, "File not found: {}", path),
            StorageError::InvalidCid(msg) => write!(f, "Invalid CID: {}", msg),
            StorageError::Io(msg) => write!(f, "IO error: {}", msg),
            StorageError::Configuration(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for StorageError {}

// Convert StorageError to String for Tauri commands
pub fn map_storage_error(err: StorageError) -> String {
    format!("{}", err)
}
