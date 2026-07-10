use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};

pub fn create_main_window(app: &AppHandle, width: f64, height: f64) -> Result<(), String> {
    WebviewWindowBuilder::new(app, "main", WebviewUrl::App("/".into()))
        .title("Meowmentum")
        .inner_size(width, height)
        .center()
        .resizable(false)
        .decorations(false)
        .visible(false)
        .build()
        .map_err(|_| "MAIN_WINDOW_BUILD_FAILED".to_string())?;

    Ok(())
}

pub fn create_drive_connect_window(app: &AppHandle) -> Result<(), String> {
    WebviewWindowBuilder::new(app, "drive-connect", WebviewUrl::App("/drive-connect".into()))
        .title("MeowMentum | Connect Your Drive")
        .inner_size(560.0, 220.0)
        .center()
        .resizable(false)
        .decorations(false)
        .build()
        .map_err(|_| "DRIVE_CONNECT_WINDOW_BUILD_FAILED".to_string())?;

    Ok(())
}