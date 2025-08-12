use cpal::{
    self,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use serde::{Deserialize, Serialize};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

enum Command {
    Start,
    Stop,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Record {
    channels: u16,
    sample_rate: u32,
    samples: Vec<f32>,
}

impl Record {
    pub fn new(channels: u16, sample_rate: u32) -> Self {
        Self {
            channels,
            sample_rate,
            samples: Vec::new(),
        }
    }

    pub fn write(&mut self, samples: &[f32]) {
        for sample in samples {
            self.samples.push(*sample);
        }
    }
}

pub struct Recorder {
    sender: mpsc::Sender<Command>,
    record: Arc<Mutex<Record>>,
}

impl Recorder {
    pub fn new() -> Self {
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

        let record = Arc::new(Mutex::new(Record::new(
            default_config.channels(),
            default_config.sample_rate().0,
        )));
        let record_clone = Arc::clone(&record);

        thread::spawn(move || {
            let stream = device
                .build_input_stream(
                    &default_config.config(),
                    move |data: &[f32], _| record_clone.lock().unwrap().write(data),
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

        Self { sender, record }
    }

    pub fn start(&self) -> Result<(), String> {
        self.sender.send(Command::Start).unwrap();
        Ok(())
    }

    pub fn stop(&self) -> Result<(), String> {
        self.sender.send(Command::Stop).unwrap();
        Ok(())
    }

    pub fn record(&self) -> Record {
        self.record.lock().unwrap().clone()
    }
}
