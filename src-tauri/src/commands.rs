use crate::recorder::{self, Recorder};
use tauri::State;

#[tauri::command]
pub fn start_recording(recorder: State<Recorder>) -> Result<(), String> {
    recorder.start()?;
    Ok(())
}

#[tauri::command]
pub fn stop_recording(recorder: State<Recorder>) -> Result<(), String> {
    recorder.stop()?;
    Ok(())
}

#[tauri::command]
pub fn play_audio(recorder: State<Recorder>) -> Result<recorder::Record, String> {
    let record = recorder.record();
    Ok(record)
}
