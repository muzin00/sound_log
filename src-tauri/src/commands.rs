use crate::player::Player;
use crate::recorder::Recorder;
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
pub fn play_audio(player: State<Player>) -> Result<(), String> {
    player.start()?;
    Ok(())
}

#[tauri::command]
pub fn stop_audio(player: State<Player>) -> Result<(), String> {
    player.stop()?;
    Ok(())
}
