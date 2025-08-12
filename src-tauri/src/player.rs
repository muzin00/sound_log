use crate::record::Record;
use cpal::{
    self,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

enum Command {
    Start,
    Stop,
}

pub struct Player {
    sender: mpsc::Sender<Command>,
}

impl Player {
    pub fn new(record: &Record) -> Self {
        let record = record.clone();
        let (sender, receiver) = mpsc::channel::<Command>();
        let receiver = Arc::new(Mutex::new(receiver));

        let host = cpal::default_host();

        // デフォルトの出力デバイスを取得
        let device = host
            .default_output_device()
            .ok_or("No default output device")
            .unwrap();

        // デバイスのデフォルト設定を取得
        let default_config = device.default_output_config().unwrap();

        thread::spawn(move || {
            let stream = device
                .build_output_stream(
                    &default_config.config(),
                    move |data: &mut [f32], _| {
                        // recordからサンプルデータを取得
                        let samples = record.read();

                        // 各サンプルを出力バッファにコピー
                        for (i, sample) in data.iter_mut().enumerate() {
                            if i < samples.len() {
                                *sample = samples[i];
                            } else {
                                // データの終端に達したら0で埋める
                                *sample = 0.0;
                            }
                        }
                    },
                    |err| eprintln!("Error: {:?}", err),
                    None,
                )
                .unwrap();

            stream.pause().unwrap();

            loop {
                let event = receiver.lock().unwrap().recv().unwrap();
                match event {
                    Command::Start => {
                        stream.play().unwrap();
                    }
                    Command::Stop => {
                        stream.pause().unwrap();
                    }
                }
            }
        });

        Self { sender }
    }

    pub fn start(&self) -> Result<(), String> {
        self.sender.send(Command::Start).unwrap();
        Ok(())
    }

    pub fn stop(&self) -> Result<(), String> {
        self.sender.send(Command::Stop).unwrap();
        Ok(())
    }
}
