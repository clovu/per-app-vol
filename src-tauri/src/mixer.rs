use std::{process::Command, sync::Arc};

use serde::Serialize;

#[cfg(target_os = "macos")]
use objc2_app_kit::{NSApplicationActivationPolicy, NSWorkspace};
use tauri::State;

use crate::audio::{device, guard::VolumeChangeGuard};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MixerState {
    system_volume: f32,
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
pub fn get_system_volume() -> Result<f32, String> {
    device::get_default_output_device_volume()
}

#[tauri::command]
pub fn get_mixer_state() -> Result<MixerState, String> {
    let output = run_osascript("get volume settings")?;
    let sys_volume = device::get_default_output_device_volume()?;

    Ok(MixerState {
        system_volume: sys_volume * 100.0,
        system_muted: parse_bool_field(&output, "output muted")?,
        apps: running_apps()?,
    })
}

#[tauri::command]
pub fn set_system_volume(
    volume: f32,
    guard: State<'_, Arc<VolumeChangeGuard>>,
) -> Result<(), String> {
    let volume = volume.min(100.0) / 100.0;
    let device_id = device::get_default_output_device_id()?;
    guard.mark();
    crate::audio::device::set_volume(volume, device_id)
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

fn _parse_number_field(settings: &str, name: &str) -> Result<u8, String> {
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
    use super::{_parse_number_field, parse_bool_field};

    const SETTINGS: &str = "output volume:42, input volume:66, alert volume:75, output muted:false";

    #[test]
    fn parses_volume_settings() {
        assert_eq!(_parse_number_field(SETTINGS, "output volume").unwrap(), 42);
        assert!(!parse_bool_field(SETTINGS, "output muted").unwrap());
    }
}
