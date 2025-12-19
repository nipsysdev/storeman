use crate::features::connection::get_storage_manager_with_handle;
use crate::features::shared::{map_storage_error, NodeInfo, StorageConnectionStatus};
use tauri::AppHandle;

#[tauri::command]
pub async fn get_node_status(app_handle: AppHandle) -> Result<StorageConnectionStatus, String> {
    let manager = get_storage_manager_with_handle(Some(app_handle))
        .await
        .map_err(map_storage_error)?;
    Ok(manager.get_status().await)
}

#[tauri::command]
pub async fn get_node_info(app_handle: AppHandle) -> Result<NodeInfo, String> {
    let manager = get_storage_manager_with_handle(Some(app_handle))
        .await
        .map_err(map_storage_error)?;
    manager.get_node_info().await.map_err(map_storage_error)
}

#[tauri::command]
pub async fn start_node(app_handle: AppHandle) -> Result<(), String> {
    let manager = get_storage_manager_with_handle(Some(app_handle))
        .await
        .map_err(map_storage_error)?;
    manager.start_node().await.map_err(map_storage_error)
}

#[tauri::command]
pub async fn stop_node(app_handle: AppHandle) -> Result<(), String> {
    let manager = get_storage_manager_with_handle(Some(app_handle))
        .await
        .map_err(map_storage_error)?;
    manager.stop_node().await.map_err(map_storage_error)
}

#[tauri::command]
pub async fn connect_to_peer(
    peer_id: String,
    addresses: Vec<String>,
    app_handle: AppHandle,
) -> Result<(), String> {
    let manager = get_storage_manager_with_handle(Some(app_handle))
        .await
        .map_err(map_storage_error)?;
    manager
        .connect_to_peer(peer_id, addresses)
        .await
        .map_err(map_storage_error)
}
