use codex_bindings::node::config::RepoKind;
use codex_bindings::{CodexConfig, LogLevel};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StoreManConfig {
    pub data_dir: PathBuf,
    pub storage_quota: u64,
    pub max_peers: u32,
    pub discovery_port: u16,
    pub log_level: LogLevel,
    pub auto_connect: bool,
}

impl Default for StoreManConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl StoreManConfig {
    pub fn new() -> Self {
        // Use a simple, reliable approach for data directory
        let data_dir = std::env::temp_dir().join("storeman").join("node_data");

        println!("Storage data directory: {}", data_dir.display());

        // Ensure the directory exists using std::fs
        if let Err(e) = std::fs::create_dir_all(&data_dir) {
            eprintln!(
                "Failed to create data directory {}: {}",
                data_dir.display(),
                e
            );
        } else {
            println!(
                "Successfully created data directory: {}",
                data_dir.display()
            );
        }

        Self {
            data_dir,
            storage_quota: 1024 * 1024 * 1024, // 1 GB
            max_peers: 50,
            discovery_port: 8089,
            log_level: LogLevel::Info,
            auto_connect: false,
        }
    }

    pub fn with_app_handle(app_handle: &AppHandle) -> Self {
        // Use app_data_dir for proper application data storage
        let data_dir = match app_handle.path().app_data_dir() {
            Ok(dir) => dir.join("node_data"),
            Err(e) => {
                eprintln!("Failed to get app data directory: {}", e);
                // Fallback to temp directory
                std::env::temp_dir().join("storeman").join("node_data")
            }
        };

        println!("Storage data directory: {}", data_dir.display());

        // Ensure the directory exists using std::fs
        if let Err(e) = std::fs::create_dir_all(&data_dir) {
            eprintln!(
                "Failed to create data directory {}: {}",
                data_dir.display(),
                e
            );
        } else {
            println!(
                "Successfully created data directory: {}",
                data_dir.display()
            );
        }

        Self {
            data_dir,
            storage_quota: 1024 * 1024 * 1024, // 1 GB
            max_peers: 50,
            discovery_port: 8089,
            log_level: LogLevel::Info,
            auto_connect: false,
        }
    }

    pub fn to_codex_config(&self) -> CodexConfig {
        CodexConfig::new()
            .log_level(self.log_level)
            .data_dir(&self.data_dir)
            .storage_quota(self.storage_quota)
            .max_peers(self.max_peers)
            .discovery_port(self.discovery_port)
            .repo_kind(RepoKind::LevelDb)
    }
}
