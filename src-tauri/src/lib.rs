mod commands;
mod main_window;

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
            commands::minimize_application
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
