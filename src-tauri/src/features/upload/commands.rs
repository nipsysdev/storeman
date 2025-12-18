use crate::features::shared::map_storage_error;
use crate::features::upload::upload_file_with_progress;
use tauri::AppHandle;

#[tauri::command]
pub async fn upload_file_to_storage(
    file_path: String,
    app_handle: AppHandle,
) -> Result<crate::features::shared::UploadResultResponse, String> {
    upload_file_with_progress(file_path.into(), app_handle)
        .await
        .map_err(map_storage_error)
}
