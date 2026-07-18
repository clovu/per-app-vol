use std::{ffi::c_void, ptr::NonNull, sync::Arc};

use objc2_core_audio::{AudioObjectID, AudioObjectPropertyAddress};

use crate::audio::{
    device::{default_output_volume_property_address, get_default_output_device_id},
    guard::{SELF_TRIGGER_WINDOW, VolumeChangeGuard},
};

/// Type-erased callback stored on the heap and passed to CoreAudio as
/// `client_data`.  CoreAudio holds a raw pointer to this allocation until the
/// listener is removed or the process exits.
type VolumeCallback = Box<dyn Fn() + Send>;

struct ListenerContext {
    listener: VolumeCallback,
    guard: Arc<VolumeChangeGuard>,
}

/// Register a listener for system output volume changes.
///
/// The provided closure is called on an arbitrary CoreAudio thread whenever
/// the default output device's volume property changes.  Events triggered by
/// this application's own [`mixer::set_system_volume`] calls are automatically
/// filtered out via the [`IGNORE_NEXT_EVENT`] guard.
///
/// The allocated callback context is intentionally leaked — it must remain
/// alive for the application's entire lifetime because CoreAudio retains a
/// raw pointer to it until the listener is removed.
pub fn register_volume_change_listener<F>(listener: F) -> Result<Arc<VolumeChangeGuard>, String>
where
    F: Fn() + Send + 'static,
{
    let guard = Arc::new(VolumeChangeGuard::new());
    let ctx = Box::new(ListenerContext {
        guard: Arc::clone(&guard),
        listener: Box::new(listener),
    });

    let device_id = get_default_output_device_id()?;
    let ctx_ptr = create_callback_context(ctx);

    install_volume_listener(device_id, ctx_ptr)?;

    Ok(guard)
}

/// Box the user-provided closure twice: the outer `Box` is the stable pointer
/// handed to CoreAudio, the inner `Box` is the type-erased callback itself.
/// Returns a `*mut c_void` suitable for passing as `client_data`.
fn create_callback_context(ctx: Box<ListenerContext>) -> *mut c_void {
    Box::into_raw(ctx).cast()
}

/// Register the volume-property listener with CoreAudio.  On failure the
/// callback context is destroyed to avoid leaking memory.
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

/// Reconstruct and drop the `VolumeCallback` from its raw pointer.  Only
/// called on the error path of [`install_volume_listener`].
unsafe fn destroy_callback_context(ptr: *mut c_void) {
    drop(unsafe { Box::from_raw(ptr as *mut VolumeCallback) });
}

/// CoreAudio property-listener callback.  Fires on an arbitrary system thread
/// whenever the default output device's volume changes.  The self-triggered
/// events (from `set_system_volume`) are filtered out via
/// [`should_ignore_volume_event`].
unsafe extern "C-unwind" fn volume_change_callback(
    _id: AudioObjectID,
    _count: u32,
    _addresses: NonNull<AudioObjectPropertyAddress>,
    client_data: *mut c_void,
) -> i32 {
    let ctx = unsafe { callback_from_client_data(client_data) };

    if ctx.guard.should_ignore(SELF_TRIGGER_WINDOW) {
        return 0;
    }

    (ctx.listener)();

    0
}

/// Cast the opaque `client_data` pointer back to a `&VolumeCallback`.  The
/// caller must ensure the pointer is valid and the underlying allocation has
/// not been dropped.
unsafe fn callback_from_client_data(client_data: *mut c_void) -> &'static ListenerContext {
    unsafe { &*(client_data as *const ListenerContext) }
}
