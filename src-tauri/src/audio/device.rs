use std::ptr::NonNull;

use objc2_audio_toolbox::kAudioHardwareServiceDeviceProperty_VirtualMainVolume;
use objc2_core_audio::{
    AudioObjectGetPropertyData, AudioObjectID, AudioObjectPropertyAddress,
    AudioObjectSetPropertyData, kAudioDevicePropertyScopeOutput,
    kAudioHardwarePropertyDefaultOutputDevice, kAudioObjectPropertyElementMain,
    kAudioObjectPropertyScopeGlobal, kAudioObjectSystemObject,
};

pub fn set_volume(vol: f32, device_id: AudioObjectID) -> Result<(), String> {
    let volume_address = default_output_volume_property_address();

    let status = unsafe {
        AudioObjectSetPropertyData(
            device_id,
            NonNull::from(&volume_address),
            0,
            std::ptr::null(),
            std::mem::size_of::<f32>() as u32,
            NonNull::from(&vol).cast(),
        )
    };

    if status != 0 {
        return Err(format!("set output device failed: {status}"));
    }

    Ok(())
}

pub fn get_default_output_device_volume() -> Result<f32, String> {
    let device_id = get_default_output_device_id().unwrap();
    let vol_address = default_output_volume_property_address();

    let mut size = std::mem::size_of::<AudioObjectID>() as u32;
    let mut vol: f32 = 0.0;

    let status = unsafe {
        AudioObjectGetPropertyData(
            device_id,
            NonNull::from(&vol_address),
            0,
            std::ptr::null(),
            NonNull::from(&mut size),
            NonNull::from(&mut vol).cast(),
        )
    };

    if status != 0 {
        return Err(format!("get default output device volume failed: {status}"));
    };

    Ok(vol)
}

fn default_output_device_property_address() -> AudioObjectPropertyAddress {
    AudioObjectPropertyAddress {
        mSelector: kAudioHardwarePropertyDefaultOutputDevice,
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMain,
    }
}

pub fn default_output_volume_property_address() -> AudioObjectPropertyAddress {
    AudioObjectPropertyAddress {
        mSelector: kAudioHardwareServiceDeviceProperty_VirtualMainVolume,
        mScope: kAudioDevicePropertyScopeOutput,
        mElement: kAudioObjectPropertyElementMain,
    }
}

pub fn get_default_output_device_id() -> Result<AudioObjectID, String> {
    let device_address = default_output_device_property_address();

    let mut device_id: AudioObjectID = 0;
    let mut size = std::mem::size_of::<AudioObjectID>() as u32;

    let status = unsafe {
        AudioObjectGetPropertyData(
            kAudioObjectSystemObject.try_into().unwrap(),
            NonNull::from(&device_address),
            0,
            std::ptr::null(),
            NonNull::from(&mut size),
            NonNull::from(&mut device_id).cast(),
        )
    };
    if status != 0 {
        return Err(format!("get output device failed: {status}"));
    };

    Ok(device_id)
}
