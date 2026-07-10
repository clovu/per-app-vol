#![allow(unexpected_cfgs)]

use tauri::ActivationPolicy;

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
        ])
        .setup(|app| {
            app.set_activation_policy(ActivationPolicy::Accessory);
            manager::popover::setup(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
