use cpal::{
    self,
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, StreamConfig,
};
use std::sync::mpsc;

pub struct Record {
    pub samples: Vec<f32>,
}

impl Record {
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
        }
    }
}

pub struct Recorder {
    pub device: Device,
    pub config: StreamConfig,
}

impl Recorder {
    pub fn new() -> Self {
        let host = cpal::default_host();

        // デフォルトの入力デバイスを取得
        let device = host
            .default_input_device()
            .ok_or("No default input device")
            .unwrap();

        // デバイスのデフォルト設定を取得
        let default_config = device.default_input_config().unwrap();
        let sample_format = default_config.sample_format();

        // サンプルフォーマットがF32でない場合はエラー
        if sample_format != cpal::SampleFormat::F32 {
            panic!("Unsupported sample format: {:?}", sample_format);
        }

        Self {
            device,
            config: default_config.config(),
        }
    }

    pub fn start(&self) -> Result<Record, String> {
        let (tx, rx) = mpsc::channel();

        // ストリームを作成
        let stream = self
            .device
            .build_input_stream(
                &self.config,
                move |data: &[f32], _info| {
                    tx.send(data.to_vec()).unwrap();
                },
                |err| eprintln!("Error: {:?}", err),
                None,
            )
            .map_err(|e| e.to_string())?;

        // ストリームを開始
        stream.play().map_err(|e| e.to_string())?;

        let mut record = Record::new();

        while let Ok(data) = rx.recv() {
            record.samples.extend(data);
            println!("Samples length: {}", record.samples.len());
        }

        Ok(record)
    }
}
