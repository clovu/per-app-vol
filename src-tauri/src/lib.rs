#![allow(unexpected_cfgs)]

use tauri::{ActivationPolicy, Emitter};

mod audio;
mod manager;
mod mixer;

// the following cfg clippy throw error with clippy in before git commit
#[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_nspopover::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            mixer::get_mixer_state,
            mixer::set_system_volume,
            mixer::get_system_volume
        ])
        .setup(|app| {
            app.set_activation_policy(ActivationPolicy::Accessory);
            manager::popover::setup(app)?;
            let app_handle = app.handle().clone();
            audio::register_volume_change_listener(move || {
                let _ = app_handle.emit("per-app-vol:show-popover", ());
            })?;
            // Leak intentionally — the listener must survive for the app's
            // entire lifetime.  CoreAudio holds a reference to the callback
            // until process exit.
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
