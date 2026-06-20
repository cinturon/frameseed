#[tauri::command]
fn welcome() -> String {
    frameseed_core::welcome_message().to_string()
}

#[tauri::command]
fn list_gallery() -> Vec<String> {
    frameseed_core::gallery_entries()
        .iter()
        .map(|entry| entry.title.to_string())
        .collect()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![welcome, list_gallery])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
