mod commands;
mod window;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    std::env::set_var(
        "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS",
        "--disable-pinch",
    );
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::create_main_window,
            commands::launch_main_window,
            commands::quit_application,
            commands::minimize_application,
            commands::set_window_always_on_top,
            commands::launch_drive_connect_window,
            commands::close_drive_connect_window
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
