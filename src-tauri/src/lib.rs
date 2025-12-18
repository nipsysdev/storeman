// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod features;

use tauri::Manager;
use tauri_plugin_fs::FsExt;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let fs = app.fs_scope();

            if let Ok(app_data_dir) = app.path().app_data_dir() {
                let storage_dir = app_data_dir.join("storage_data");
                fs.allow_directory(&storage_dir, true)
                    .expect("Failed to allow Storage data directory");
                println!("Allowed Storage data directory: {}", storage_dir.display());
            }

            if let Ok(app_local_data_dir) = app.path().app_local_data_dir() {
                let storage_local_dir = app_local_data_dir.join("storage_data");
                fs.allow_directory(&storage_local_dir, true)
                    .expect("Failed to allow Storage local data directory");
                println!(
                    "Allowed Storage local data directory: {}",
                    storage_local_dir.display()
                );
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            features::connection::get_storage_status,
            features::connection::get_storage_error,
            features::connection::get_network_info,
            features::connection::get_storage_info,
            features::connection::connect_to_storage,
            features::connection::disconnect_from_storage,
            features::connection::get_storage_peer_id,
            features::connection::get_storage_version,
            features::upload::upload_file_to_storage,
            features::download::download_file_from_storage,
            features::connection::connect_to_peer,
            features::connection::get_node_addresses
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
