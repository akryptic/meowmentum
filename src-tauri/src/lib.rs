mod commands;
mod main_window;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::create_main_window,
            commands::launch_main_window,
            commands::quit_application,
            commands::minimize_application
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
