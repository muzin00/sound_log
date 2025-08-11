use crate::recorder::Recorder;
use cpal::{
    self,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use std::sync::mpsc;
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
    let (tx, rx) = mpsc::channel();

    // ストリームを作成
    let stream = recorder
        .device
        .build_input_stream(
            &recorder.config,
            move |data: &[f32], _info| {
                tx.send(data.to_vec()).unwrap();
            },
            |err| eprintln!("Error: {:?}", err),
            None,
        )
        .map_err(|e| e.to_string())?;

    // ストリームを開始
    stream.play().map_err(|e| e.to_string())?;

    println!("Recording started...");

    let mut samples = Vec::<f32>::new();

    while let Ok(data) = rx.recv() {
        samples.extend(data);

        println!("Samples length: {}", samples.len());

        if samples.len() > 960000 {
            break;
        };
    }

    Ok(())
}

// #[test]
// fn test_start_recording() {
//     let audio_data = start_recording().unwrap();
//     println!("Audio data: {:?}", audio_data);
// }
