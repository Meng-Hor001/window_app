#[cfg_attr(mobile, tauri::mobile_entry_point)]

#[tauri::command]
fn check_connection() -> String {
    erd_core::connection_message()
}

#[tauri::command]
fn get_sample_schema() -> erd_core::model::DatabaseSchema {
    erd_core::sample_schema()
}

pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
    check_connection,
    get_sample_schema
])
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
    .expect("error while building tauri application");
}
