use codex_bindings::{connect, debug, CodexNode};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, OnceCell, RwLock};

use crate::features::shared::{NetworkInfo, StorageConnectionStatus, StorageError, StorageInfo};

pub struct StorageManager {
    node: Arc<Mutex<Option<CodexNode>>>,
    config: crate::features::connection::StoreManConfig,
    status: Arc<RwLock<StorageConnectionStatus>>,
    error: Arc<RwLock<Option<String>>>,
    progress_senders: Arc<
        Mutex<
            HashMap<
                String,
                tokio::sync::mpsc::UnboundedSender<crate::features::shared::ProgressMessage>,
            >,
        >,
    >,
    network_info: Arc<RwLock<NetworkInfo>>,
    storage_info: Arc<RwLock<StorageInfo>>,
}

impl StorageManager {
    pub async fn new(
        config: crate::features::connection::StoreManConfig,
    ) -> Result<Self, StorageError> {
        let manager = Self {
            node: Arc::new(Mutex::new(None)),
            config,
            status: Arc::new(RwLock::new(StorageConnectionStatus::Disconnected)),
            error: Arc::new(RwLock::new(None)),
            progress_senders: Arc::new(Mutex::new(HashMap::new())),
            network_info: Arc::new(RwLock::new(NetworkInfo {
                peer_id: None,
                version: None,
                repo_path: None,
                connected_peers: 0,
                max_peers: 50,
            })),
            storage_info: Arc::new(RwLock::new(StorageInfo {
                used_bytes: 0,
                total_bytes: 1024 * 1024 * 1024,
                available_bytes: 1024 * 1024 * 1024,
                block_count: 0,
            })),
        };

        manager.initialize_node().await?;

        if manager.config.auto_connect {
            manager.start_node().await?;
        }

        Ok(manager)
    }

    pub async fn initialize_node(&self) -> Result<(), StorageError> {
        {
            let mut status = self.status.write().await;
            *status = StorageConnectionStatus::Connecting;
        }

        // Clear any previous errors
        {
            let mut error = self.error.write().await;
            *error = None;
        }

        {
            let node_guard = self.node.lock().await;
            if node_guard.is_some() {
                // Node already initialized, just update status
                let mut status = self.status.write().await;
                *status = StorageConnectionStatus::Initialized;
                return Ok(());
            }
        }

        let storage_config = self.config.to_codex_config();

        let node = match CodexNode::new(storage_config) {
            Ok(node) => node,
            Err(e) => {
                return Err(StorageError::NodeCreation(e.to_string()));
            }
        };

        {
            let mut node_guard = self.node.lock().await;
            *node_guard = Some(node);
        }

        {
            let mut status = self.status.write().await;
            *status = StorageConnectionStatus::Initialized;
        }

        Ok(())
    }

    pub async fn start_node(&self) -> Result<(), StorageError> {
        {
            let mut status = self.status.write().await;
            *status = StorageConnectionStatus::Connecting;
        }

        {
            let mut error = self.error.write().await;
            *error = None;
        }

        let mut node = {
            let mut node_guard = self.node.lock().await;
            match node_guard.take() {
                Some(node) => node,
                None => {
                    // Node not initialized, initialize it first
                    drop(node_guard);
                    self.initialize_node().await?;
                    let mut node_guard = self.node.lock().await;
                    node_guard
                        .take()
                        .ok_or_else(|| StorageError::NodeNotInitialized)?
                }
            }
        };

        match node.start() {
            Ok(_) => {}
            Err(e) => {
                let mut node_guard = self.node.lock().await;
                *node_guard = Some(node);
                return Err(StorageError::NodeStart(e.to_string()));
            }
        }

        {
            let mut node_guard = self.node.lock().await;
            *node_guard = Some(node);
        }

        {
            let mut status = self.status.write().await;
            *status = StorageConnectionStatus::Connected;
        }

        // Update network info
        self.update_network_info().await?;

        Ok(())
    }

    pub async fn stop_node(&self) -> Result<(), StorageError> {
        {
            let mut status = self.status.write().await;
            *status = StorageConnectionStatus::Disconnected;
        }

        {
            let node_option = {
                let mut node_guard = self.node.lock().await;
                node_guard.take()
            };

            if let Some(mut node) = node_option {
                if let Err(e) = node.stop() {
                    eprintln!("Failed to stop node: {}", e);
                }
                // Put the stopped node back
                let mut node_guard = self.node.lock().await;
                *node_guard = Some(node);
            }
        }

        {
            let mut network_info = self.network_info.write().await;
            network_info.peer_id = None;
            network_info.version = None;
            network_info.repo_path = None;
            network_info.connected_peers = 0;
        }

        {
            let mut status = self.status.write().await;
            *status = StorageConnectionStatus::Initialized;
        }

        {
            let mut error = self.error.write().await;
            *error = None;
        }

        Ok(())
    }

    pub async fn get_status(&self) -> StorageConnectionStatus {
        self.status.read().await.clone()
    }

    pub async fn get_error(&self) -> Option<String> {
        self.error.read().await.clone()
    }

    pub async fn get_network_info(&self) -> NetworkInfo {
        self.network_info.read().await.clone()
    }

    pub async fn get_storage_info(&self) -> StorageInfo {
        self.storage_info.read().await.clone()
    }

    pub async fn connect_to_peer(
        &self,
        peer_id: String,
        addresses: Vec<String>,
    ) -> Result<(), StorageError> {
        // Get the node (existing pattern from upload/download methods)
        let node = {
            let node_guard = self.node.lock().await;
            node_guard
                .as_ref()
                .ok_or_else(|| StorageError::NodeNotInitialized)?
                .clone()
        };

        if !node.is_started() {
            return Err(StorageError::NodeNotStarted);
        }

        connect(&node, &peer_id, &addresses)
            .await
            .map_err(|e| StorageError::Configuration(e.to_string()))?;

        Ok(())
    }

    pub async fn get_node_addresses(&self) -> Result<Vec<String>, StorageError> {
        // Get the node (existing pattern)
        let node = {
            let node_guard = self.node.lock().await;
            node_guard
                .as_ref()
                .ok_or_else(|| StorageError::NodeNotInitialized)?
                .clone()
        };

        if !node.is_started() {
            return Err(StorageError::NodeNotStarted);
        }

        let debug_info = debug(&node)
            .await
            .map_err(|e| StorageError::Configuration(e.to_string()))?;
        Ok(debug_info.addrs)
    }

    async fn update_network_info(&self) -> Result<(), StorageError> {
        let node = {
            let node_guard = self.node.lock().await;
            node_guard
                .as_ref()
                .ok_or_else(|| StorageError::NodeNotInitialized)?
                .clone()
        };

        let mut network_info = self.network_info.write().await;

        network_info.peer_id = node.peer_id().ok();
        network_info.version = node.version().ok();
        network_info.repo_path = node.repo().ok();
        network_info.max_peers = self.config.max_peers;
        // TODO: Get actual connected peers count when available in bindings

        Ok(())
    }

    // Helper methods for upload/download features
    pub async fn get_node(&self) -> Result<CodexNode, StorageError> {
        let node_guard = self.node.lock().await;
        node_guard
            .as_ref()
            .ok_or_else(|| StorageError::NodeNotInitialized)
            .map(|node| node.clone())
    }

    pub async fn send_progress(
        &self,
        operation_id: &str,
        progress: crate::features::shared::ProgressMessage,
    ) {
        let senders = self.progress_senders.lock().await;
        if let Some(sender) = senders.get(operation_id) {
            let _ = sender.send(progress);
        }
    }

    pub async fn register_progress_sender(
        &self,
        operation_id: String,
    ) -> tokio::sync::mpsc::UnboundedReceiver<crate::features::shared::ProgressMessage> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        {
            let mut senders = self.progress_senders.lock().await;
            senders.insert(operation_id, tx);
        }
        rx
    }

    pub async fn unregister_progress_sender(&self, operation_id: &str) {
        let mut senders = self.progress_senders.lock().await;
        senders.remove(operation_id);
    }
}

impl Clone for StorageManager {
    fn clone(&self) -> Self {
        Self {
            node: Arc::clone(&self.node),
            config: self.config.clone(),
            status: Arc::clone(&self.status),
            error: Arc::clone(&self.error),
            progress_senders: Arc::clone(&self.progress_senders),
            network_info: Arc::clone(&self.network_info),
            storage_info: Arc::clone(&self.storage_info),
        }
    }
}

// Global manager instance
pub static STORAGE_MANAGER: OnceCell<Arc<StorageManager>> = OnceCell::const_new();

pub async fn get_storage_manager_with_handle(
    app_handle: Option<tauri::AppHandle>,
) -> Result<Arc<StorageManager>, StorageError> {
    if let Some(manager) = STORAGE_MANAGER.get() {
        Ok(Arc::clone(manager))
    } else {
        let config = if let Some(handle) = app_handle {
            crate::features::connection::StoreManConfig::with_app_handle(&handle)
        } else {
            crate::features::connection::StoreManConfig::new()
        };
        let manager = Arc::new(StorageManager::new(config).await?);
        STORAGE_MANAGER.set(manager.clone()).map_err(|_| {
            StorageError::Configuration("Failed to initialize Storage manager".to_string())
        })?;
        Ok(manager)
    }
}
