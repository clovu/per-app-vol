use std::{process::Command, ptr::NonNull};

use objc2_audio_toolbox::kAudioHardwareServiceDeviceProperty_VirtualMainVolume;
use objc2_core_audio::{
    AudioObjectGetPropertyData, AudioObjectID, AudioObjectPropertyAddress,
    AudioObjectSetPropertyData, kAudioDevicePropertyScopeOutput,
    kAudioHardwarePropertyDefaultOutputDevice, kAudioObjectPropertyElementMain,
    kAudioObjectPropertyScopeGlobal, kAudioObjectSystemObject,
};
use serde::Serialize;

#[cfg(target_os = "macos")]
use objc2_app_kit::{NSApplicationActivationPolicy, NSWorkspace};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MixerState {
    system_volume: u8,
    system_muted: bool,
    apps: Vec<RunningApp>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunningApp {
    id: String,
    name: String,
    pid: u32,
    volume: u8,
    muted: bool,
    controllable: bool,
}

#[tauri::command]
pub fn get_mixer_state() -> Result<MixerState, String> {
    let output = run_osascript("get volume settings")?;

    Ok(MixerState {
        system_volume: parse_number_field(&output, "output volume")?,
        system_muted: parse_bool_field(&output, "output muted")?,
        apps: running_apps()?,
    })
}

fn default_output_device_property_address() -> AudioObjectPropertyAddress {
    AudioObjectPropertyAddress {
        mSelector: kAudioHardwarePropertyDefaultOutputDevice,
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMain,
    }
}

fn default_output_volume_property_address() -> AudioObjectPropertyAddress {
    AudioObjectPropertyAddress {
        mSelector: kAudioHardwareServiceDeviceProperty_VirtualMainVolume,
        mScope: kAudioDevicePropertyScopeOutput,
        mElement: kAudioObjectPropertyElementMain,
    }
}

fn get_default_output_device_id() -> Result<AudioObjectID, String> {
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

fn set_volume(vol: f32, device_id: AudioObjectID) -> Result<(), String> {
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
        return Err(format!("get output device failed: {status}"));
    }

    Ok(())
}

#[tauri::command]
pub fn set_system_volume(volume: u8) -> Result<(), String> {
    let volume = f32::from(volume.min(100)) / 100.0;
    let device_id = get_default_output_device_id()?;
    set_volume(volume, device_id)
}

fn run_osascript(script: &str) -> Result<String, String> {
    let output = Command::new("/usr/bin/osascript")
        .args(["-e", script])
        .output()
        .map_err(|error| format!("failed to run osascript: {error}"))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_owned());
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

#[cfg(target_os = "macos")]
fn running_apps() -> Result<Vec<RunningApp>, String> {
    let workspace = NSWorkspace::sharedWorkspace();
    let running = workspace.runningApplications();

    let mut apps = running
        .iter()
        .filter(|application| {
            application.activationPolicy() == NSApplicationActivationPolicy::Regular
                && !application.isTerminated()
                && application.bundleURL().is_some()
                && application.processIdentifier() > 0
        })
        .filter_map(|application| {
            let pid = application.processIdentifier() as u32;
            let name = application.localizedName()?.to_string();

            Some(RunningApp {
                id: format!("pid:{pid}"),
                name,
                pid,
                volume: 100,
                muted: false,
                controllable: false,
            })
        })
        .collect::<Vec<_>>();

    apps.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(apps)
}

#[cfg(not(target_os = "macos"))]
fn running_apps() -> Result<Vec<RunningApp>, String> {
    Ok(Vec::new())
}

fn parse_number_field(settings: &str, name: &str) -> Result<u8, String> {
    field_value(settings, name)?
        .parse()
        .map_err(|error| format!("invalid {name}: {error}"))
}

fn parse_bool_field(settings: &str, name: &str) -> Result<bool, String> {
    field_value(settings, name)?
        .parse()
        .map_err(|error| format!("invalid {name}: {error}"))
}

fn field_value<'a>(settings: &'a str, name: &str) -> Result<&'a str, String> {
    settings
        .split(',')
        .map(str::trim)
        .find_map(|field| field.strip_prefix(&format!("{name}:")))
        .map(str::trim)
        .ok_or_else(|| format!("missing {name} in volume settings"))
}

#[cfg(test)]
mod tests {
    use super::{parse_bool_field, parse_number_field};

    const SETTINGS: &str = "output volume:42, input volume:66, alert volume:75, output muted:false";

    #[test]
    fn parses_volume_settings() {
        assert_eq!(parse_number_field(SETTINGS, "output volume").unwrap(), 42);
        assert!(!parse_bool_field(SETTINGS, "output muted").unwrap());
    }
}
