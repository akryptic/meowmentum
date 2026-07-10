use serde::Serialize;
use tauri::{AppHandle, Manager, WebviewWindow};

use crate::window;

#[derive(Serialize)]
struct AppError {
    code: &'static str,
    message: &'static str,
    recoverable: bool,
    context: Option<ErrorContext>,
}

#[derive(Serialize)]
struct ErrorContext {
    stage: &'static str,
    hint: &'static str,
}

fn app_error(
    code: &'static str,
    message: &'static str,
    recoverable: bool,
    stage: &'static str,
    hint: &'static str,
) -> String {
    let error = AppError {
        code,
        message,
        recoverable,
        context: Some(ErrorContext { stage, hint }),
    };

    serde_json::to_string_pretty(&error).unwrap_or_else(|_| {
        r#"{"code":"UNKNOWN_ERROR","message":"Something went wrong.","recoverable":false}"#.into()
    })
}

#[tauri::command]
pub async fn create_main_window(
    app: AppHandle,
    window_width: f64,
    window_height: f64,
) -> Result<(), String> {
    if app.get_webview_window("main").is_some() {
        return Ok(());
    }

    if window_width <= 0.0 || window_height <= 0.0 {
        return Err(app_error(
            "INVALID_WINDOW_SIZE",
            "The application could not calculate a valid window size.",
            false,
            "create_main_window",
            "Please report this error to the developer.",
        ));
    }

    window::create_main_window(&app, window_width, window_height).map_err(|_| {
        app_error(
            "MAIN_WINDOW_CREATE_FAILED",
            "The main application window could not be created.",
            false,
            "create_main_window",
            "Please report this error to the developer.",
        )
    })?;

    Ok(())
}

#[tauri::command]
pub async fn launch_main_window(app: AppHandle) -> Result<(), String> {
    let setup = app.get_webview_window("setup").ok_or_else(|| {
        app_error(
            "SETUP_WINDOW_NOT_FOUND",
            "The setup window was not found.",
            false,
            "launch_main_window",
            "Please restart the application.",
        )
    })?;

    let main = app.get_webview_window("main").ok_or_else(|| {
        app_error(
            "MAIN_WINDOW_NOT_FOUND",
            "The main window was not ready to launch.",
            false,
            "launch_main_window",
            "Please restart the application.",
        )
    })?;

    main.show().map_err(|_| {
        app_error(
            "MAIN_WINDOW_SHOW_FAILED",
            "The main window could not be shown.",
            false,
            "launch_main_window",
            "Please report this error to the developer.",
        )
    })?;

    setup.close().map_err(|_| {
        app_error(
            "SETUP_WINDOW_CLOSE_FAILED",
            "The setup window could not be closed.",
            true,
            "launch_main_window",
            "The app may still be usable.",
        )
    })?;

    Ok(())
}

#[tauri::command]
pub async fn launch_drive_connect_window(app: AppHandle) -> Result<(), String> {
    if app.get_webview_window("drive-connect").is_some() {
        return Ok(());
    }

    window::create_drive_connect_window(&app).map_err(|_| {
        app_error(
            "DRIVE_CONNECT_WINDOW_CREATE_FAILED",
            "The drive connection window could not be created.",
            false,
            "create_drive_connect_window",
            "Please report this error to the developer.",
        )
    })?;

    let main = app.get_webview_window("main").ok_or_else(|| {
        app_error(
            "MAIN_WINDOW_NOT_FOUND",
            "The main window was not ready to launch.",
            false,
            "launch_main_window",
            "Please restart the application.",
        )
    })?;

    main.hide().map_err(|_| {
        app_error(
            "MAIN_WINDOW_HIDE_FAILED",
            "The main window could not be hidden.",
            false,
            "launch_drive_connect_window",
            "Please report this error to the developer.",
        )
    })?;

    Ok(())
}

#[tauri::command]
pub fn quit_application(app: AppHandle) {
    app.exit(1);
}

#[tauri::command]
pub fn minimize_application(window: WebviewWindow) -> Result<(), String> {
    window.minimize().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_window_always_on_top(window: WebviewWindow, flag: bool) -> Result<(), String> {
    window.set_always_on_top(flag).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn close_drive_connect_window(window: WebviewWindow, app: AppHandle) -> Result<(), String> {
    let main = app.get_webview_window("main").ok_or_else(|| {
        app_error(
            "MAIN_WINDOW_NOT_FOUND",
            "The main window was not ready to launch.",
            false,
            "launch_main_window",
            "Please restart the application.",
        )
    })?;

    main.show().map_err(|_| {
        app_error(
            "MAIN_WINDOW_HIDE_FAILED",
            "The main window could not be hidden.",
            false,
            "launch_drive_connect_window",
            "Please report this error to the developer.",
        )
    })?;

    window.close().map_err(|_| {
        app_error(
            "SETUP_WINDOW_CLOSE_FAILED",
            "The setup window could not be closed.",
            true,
            "launch_main_window",
            "The app may still be usable.",
        )
    })?;

    Ok(())
}
