use std::{thread, time};

use cpal::{
    self,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};

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
pub fn start_recording() -> Result<(), String> {
    let host = cpal::default_host();

    // デフォルトの入力デバイスを取得
    let device = host
        .default_input_device()
        .ok_or("No default input device")?;

    // デバイスのデフォルト設定を取得
    let config = device.default_input_config().map_err(|e| e.to_string())?;

    println!("Recording with config: {:?}", config);

    // サンプルフォーマットがF32でない場合はエラー
    if config.sample_format() != cpal::SampleFormat::F32 {
        return Err("Unsupported sample format".to_string());
    }

    // ストリームを作成
    let stream = device
        .build_input_stream(
            &config.into(),
            move |data: &[f32], _| {
                println!("Received data: {:?}", data);
            },
            |err| eprintln!("Error: {:?}", err),
            None,
        )
        .map_err(|e| e.to_string())?;

    // ストリームを開始
    stream.play().map_err(|e| e.to_string())?;

    println!("Recording started...");

    thread::sleep(time::Duration::from_secs(10));

    Ok(())
}
