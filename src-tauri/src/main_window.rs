use tauri::{AppHandle, WebviewUrl, WebviewWindowBuilder};

pub fn create(app: &AppHandle, width: f64, height: f64) -> Result<(), String> {
    WebviewWindowBuilder::new(app, "main", WebviewUrl::App("/".into()))
        .title("Meomentum")
        .inner_size(width, height)
        .center()
        .resizable(false)
        .decorations(false)
        .visible(false)
        .build()
        .map_err(|_| "MAIN_WINDOW_BUILD_FAILED".to_string())?;

    Ok(())
}