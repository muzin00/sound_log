use std::sync::mpsc;

use cpal::{
    self,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};

use serde::{Deserialize, Serialize};

#[tauri::command]
pub fn get_input_devices() -> Vec<String> {
    let host = cpal::default_host();
    let devices = host.input_devices().unwrap();
    devices
        .into_iter()
        .map(|device| device.name().unwrap())
        .collect()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AudioData {
    channels: u16,
    sample_rate: u32,
    samples: Vec<f32>,
}

#[tauri::command]
pub fn start_recording() -> Result<AudioData, String> {
    let host = cpal::default_host();

    // デフォルトの入力デバイスを取得
    let device = host
        .default_input_device()
        .ok_or("No default input device")?;

    // デバイスのデフォルト設定を取得
    let supported_config = device.default_input_config().map_err(|e| e.to_string())?;

    println!("Recording with supported config: {:?}", supported_config);

    // サンプルフォーマットがF32でない場合はエラー
    if supported_config.sample_format() != cpal::SampleFormat::F32 {
        return Err("Unsupported sample format".to_string());
    }

    let config = supported_config.into();

    let (tx, rx) = mpsc::channel();

    // ストリームを作成
    let stream = device
        .build_input_stream(
            &config,
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

    Ok(AudioData {
        channels: config.channels,
        sample_rate: config.sample_rate.0,
        samples,
    })
}

// #[test]
// fn test_start_recording() {
//     let audio_data = start_recording().unwrap();
//     println!("Audio data: {:?}", audio_data);
// }
