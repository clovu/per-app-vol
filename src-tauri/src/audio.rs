use std::{ffi::c_void, ptr::NonNull, time::Duration};

use objc2_core_audio::{AudioObjectID, AudioObjectPropertyAddress};

use crate::mixer::{
    IGNORE_NEXT_EVENT, default_output_volume_property_address, get_default_output_device_id,
};

type VolumeCallback = Box<dyn Fn() + Send>;

/// Register a listener for system output volume changes.
pub fn register_volume_change_listener<F>(listener: F) -> Result<(), String>
where
    F: Fn() + Send + 'static,
{
    let device_id = get_default_output_device_id()?;

    let callback_context = create_callback_context(listener);

    install_volume_listener(device_id, callback_context)
}

fn create_callback_context<F>(listener: F) -> *mut c_void
where
    F: Fn() + Send + 'static,
{
    let callback: VolumeCallback = Box::new(listener);

    Box::into_raw(Box::new(callback)).cast()
}

fn install_volume_listener(device_id: AudioObjectID, context: *mut c_void) -> Result<(), String> {
    let address = default_output_volume_property_address();

    let status = unsafe {
        objc2_core_audio::AudioObjectAddPropertyListener(
            device_id,
            NonNull::from(&address),
            Some(volume_change_callback),
            context,
        )
    };

    if status != 0 {
        unsafe {
            destroy_callback_context(context);
        }

        return Err(format!("failed to register volume listener ({status})"));
    }

    Ok(())
}

unsafe fn destroy_callback_context(ptr: *mut c_void) {
    drop(unsafe { Box::from_raw(ptr as *mut VolumeCallback) });
}

unsafe extern "C-unwind" fn volume_change_callback(
    _id: AudioObjectID,
    _count: u32,
    _addresses: NonNull<AudioObjectPropertyAddress>,
    client_data: *mut c_void,
) -> i32 {
    if should_ignore_volume_event() {
        return 0;
    }

    let callback = unsafe { callback_from_client_data(client_data) };

    callback();

    0
}

unsafe fn callback_from_client_data(client_data: *mut c_void) -> &'static VolumeCallback {
    unsafe { &*(client_data as *const VolumeCallback) }
}

fn should_ignore_volume_event() -> bool {
    let mut guard = IGNORE_NEXT_EVENT.lock().unwrap();

    match *guard {
        Some(timestamp) if timestamp.elapsed() < Duration::from_millis(20) => true,

        Some(_) => {
            *guard = None;
            false
        }

        None => false,
    }
}
