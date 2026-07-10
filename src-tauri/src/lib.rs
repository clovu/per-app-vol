#![allow(unexpected_cfgs)]

use std::{ffi::c_void, ptr::NonNull, sync::Mutex, time::Duration};

use objc2_core_audio::{AudioObjectID, AudioObjectPropertyAddress};
use tauri::{ActivationPolicy, Emitter, Manager};

use crate::mixer::{
    IGNORE_NEXT_EVENT, default_output_volume_property_address, get_default_output_device_id,
};

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

            unsafe extern "C-unwind" fn listener(
                _id: AudioObjectID,
                _number_addresses: u32,
                _addresses: NonNull<AudioObjectPropertyAddress>,
                client_data: *mut c_void,
            ) -> i32 {
                // ignore the event if it is triggered by set_system_volume
                let mut state = IGNORE_NEXT_EVENT.lock().unwrap();
                if let Some(state) = *state
                    && state.elapsed() < Duration::from_millis(20)
                {
                    return 0;
                }

                if state.is_some() {
                    *state = None;
                }

                let app_handle = unsafe { &*(client_data as *const Mutex<tauri::AppHandle>) };

                if let Ok(app_handle) = app_handle.lock() {
                    let _ = app_handle.emit("per-app-vol:show-popover", ());
                }

                0
            }

            let volume_address = default_output_volume_property_address();
            // let mut app_handle = Box::into_raw(Box::new(app.handle().clone()));
            let app_handle = Box::new(Mutex::new(app.app_handle().clone()));
            let app_handle_ptr = Box::into_raw(app_handle).cast();

            unsafe {
                objc2_core_audio::AudioObjectAddPropertyListener(
                    get_default_output_device_id().unwrap(),
                    NonNull::from(&volume_address),
                    Some(listener),
                    app_handle_ptr,
                )
            };
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
