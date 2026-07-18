use anyhow::{Context, Result};
use tauri::{
    App, AppHandle, Manager,
    tray::{MouseButton, MouseButtonState, TrayIconEvent},
};
use tauri_plugin_nspopover::{AppExt, ToPopoverOptions, WindowExt};

const MAIN_TRAY_ID: &str = "main";
const MAIN_WINDOW_LABEL: &str = "main";

pub fn setup(app: &mut App) -> Result<()> {
    let tray = app
        .tray_by_id(MAIN_TRAY_ID)
        .context("main tray icon is not configured")?;
    let window = app
        .get_webview_window(MAIN_WINDOW_LABEL)
        .context("main webview window is not configured")?;

    window.to_popover(ToPopoverOptions {
        is_fullsize_content: false,
    });

    let app_handle = app.handle().clone();
    tray.on_tray_icon_event(move |_, event| {
        if is_primary_click(&event) {
            toggle(&app_handle);
        }
    });

    Ok(())
}

fn is_primary_click(event: &TrayIconEvent) -> bool {
    matches!(
        event,
        TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        }
    )
}

fn toggle(app_handle: &AppHandle) {
    if app_handle.is_popover_shown() {
        app_handle.hide_popover();
    } else {
        app_handle.show_popover();
    }
}
