use crate::recorder::Recorder;
use cpal::{
    self,
    traits::{DeviceTrait, HostTrait},
};
use tauri::State;

#[tauri::command]
pub fn get_input_devices() -> Vec<String> {
    let host = cpal::default_host();
    let devices = host.input_devices().unwrap();
    devices
        .into_iter()
        .map(|device| device.name().unwrap())
        .collect()
}

#[tauri::command]
pub fn start_recording(recorder: State<Recorder>) -> Result<(), String> {
    recorder.start()?;
    Ok(())
}
