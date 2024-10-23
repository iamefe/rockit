// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("Starting Tauri application"); // Added logging
    println!("Registering commands..."); // Added logging

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::create_task,
            commands::fetch_tasks,
            commands::fetch_task_by_id,
            commands::edit_task,
            commands::remove_task
        ])
        .setup(|_app| {
            println!("Tauri application setup complete"); // Added logging
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
