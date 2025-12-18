use crate::features::download::download_file_with_progress;
use crate::features::shared::map_storage_error;
use tauri::AppHandle;

#[tauri::command]
pub async fn download_file_from_storage(
    cid: String,
    save_path: String,
    app_handle: AppHandle,
) -> Result<crate::features::shared::DownloadResultResponse, String> {
    download_file_with_progress(cid, save_path.into(), app_handle)
        .await
        .map_err(map_storage_error)
}
