use crate::features::connection::get_storage_manager_with_handle;
use crate::features::shared::{
    map_storage_error, NetworkInfo, StorageConnectionStatus, StorageInfo,
};
use tauri::AppHandle;

#[tauri::command]
pub async fn get_storage_status(app_handle: AppHandle) -> Result<StorageConnectionStatus, String> {
    let manager = get_storage_manager_with_handle(Some(app_handle))
        .await
        .map_err(map_storage_error)?;
    Ok(manager.get_status().await)
}

#[tauri::command]
pub async fn get_storage_error(app_handle: AppHandle) -> Result<Option<String>, String> {
    let manager = get_storage_manager_with_handle(Some(app_handle))
        .await
        .map_err(map_storage_error)?;
    Ok(manager.get_error().await)
}

#[tauri::command]
pub async fn get_network_info(app_handle: AppHandle) -> Result<NetworkInfo, String> {
    let manager = get_storage_manager_with_handle(Some(app_handle))
        .await
        .map_err(map_storage_error)?;
    Ok(manager.get_network_info().await)
}

#[tauri::command]
pub async fn get_storage_info(app_handle: AppHandle) -> Result<StorageInfo, String> {
    let manager = get_storage_manager_with_handle(Some(app_handle))
        .await
        .map_err(map_storage_error)?;
    Ok(manager.get_storage_info().await)
}

#[tauri::command]
pub async fn start_storage_node(app_handle: AppHandle) -> Result<(), String> {
    let manager = get_storage_manager_with_handle(Some(app_handle))
        .await
        .map_err(map_storage_error)?;
    manager.start_node().await.map_err(map_storage_error)
}

#[tauri::command]
pub async fn stop_storage_node(app_handle: AppHandle) -> Result<(), String> {
    let manager = get_storage_manager_with_handle(Some(app_handle))
        .await
        .map_err(map_storage_error)?;
    manager.stop_node().await.map_err(map_storage_error)
}

#[tauri::command]
pub async fn get_storage_peer_id(app_handle: AppHandle) -> Result<Option<String>, String> {
    let manager = get_storage_manager_with_handle(Some(app_handle))
        .await
        .map_err(map_storage_error)?;
    let network_info = manager.get_network_info().await;
    Ok(network_info.peer_id)
}

#[tauri::command]
pub async fn get_storage_version(app_handle: AppHandle) -> Result<Option<String>, String> {
    let manager = get_storage_manager_with_handle(Some(app_handle))
        .await
        .map_err(map_storage_error)?;
    let network_info = manager.get_network_info().await;
    Ok(network_info.version)
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

#[tauri::command]
pub async fn get_node_addresses(app_handle: AppHandle) -> Result<Vec<String>, String> {
    let manager = get_storage_manager_with_handle(Some(app_handle))
        .await
        .map_err(map_storage_error)?;
    manager
        .get_node_addresses()
        .await
        .map_err(map_storage_error)
}
