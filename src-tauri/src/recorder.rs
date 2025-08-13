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

pub struct Recorder {
    sender: mpsc::Sender<Command>,
}

impl Recorder {
    pub fn new(record: &mut Record) -> Self {
        let (sender, receiver) = mpsc::channel::<Command>();
        let receiver = Arc::new(Mutex::new(receiver));

        let host = cpal::default_host();

        // デフォルトの入力デバイスを取得
        let device = host
            .default_input_device()
            .ok_or("No default input device")
            .unwrap();

        // デバイスのデフォルト設定を取得
        let default_config = device.default_input_config().unwrap();

        // 録音データの設定を更新
        record.channels = default_config.channels();
        record.sample_rate = default_config.sample_rate().0;

        let record = record.clone();

        thread::spawn(move || {
            let stream = device
                .build_input_stream(
                    &default_config.config(),
                    move |data: &[f32], _| record.write(data),
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
