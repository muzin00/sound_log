use cpal::{
    self,
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, StreamConfig,
};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

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
    device: Device,
    config: StreamConfig,
    record: Arc<Mutex<Record>>,
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
            record: Arc::new(Mutex::new(Record::new())),
        }
    }

    pub fn start(&self) -> Result<(), String> {
        let device = self.device.clone();
        let config = self.config.clone();
        let record = Arc::clone(&self.record);

        thread::spawn(move || {
            // チャネルを作成
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
                .unwrap();

            // ストリームを開始
            stream.play().unwrap();

            // スレッドを作成してデータを受信
            while let Ok(data) = rx.recv() {
                let mut record = record.lock().unwrap();
                record.samples.extend(data);
            }
        });

        Ok(())
    }
}
