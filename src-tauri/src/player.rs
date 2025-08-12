use crate::record::Record;
use cpal::{
    self,
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BufferSize, SampleRate, StreamConfig,
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

        let stream_config = StreamConfig {
            channels: 1,
            sample_rate: SampleRate(48000),
            buffer_size: BufferSize::Default,
        };

        thread::spawn(move || {
            let mut playback_position = 0usize;

            let stream = device
                .build_output_stream(
                    &stream_config,
                    move |data: &mut [f32], _| {
                        // recordからサンプルデータを取得
                        let samples = record.read();

                        // 各サンプルを出力バッファにコピー
                        for sample in data.iter_mut() {
                            if playback_position < samples.len() {
                                *sample = samples[playback_position];
                                playback_position += 1;
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
