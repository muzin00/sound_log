use cpal::{
    self,
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, StreamConfig,
};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

enum RecordEvent {
    Start,
    Stop,
}

struct Record {
    thread: thread::JoinHandle<()>,
    samples: Arc<Mutex<Vec<f32>>>,
}

impl Record {
    fn new(
        receiver: Arc<Mutex<mpsc::Receiver<RecordEvent>>>,
        device: Device,
        config: StreamConfig,
    ) -> Self {
        let samples = Arc::new(Mutex::new(Vec::new()));
        let samples_clone = Arc::clone(&samples);

        let thread = thread::spawn(move || {
            let stream = device
                .build_input_stream(
                    &config,
                    move |data: &[f32], _info| {
                        let mut samples = samples_clone.lock().unwrap();
                        samples.extend(data);
                        println!("Received {} samples", samples.len());
                    },
                    |err| eprintln!("Error: {:?}", err),
                    None,
                )
                .unwrap();

            stream.pause().unwrap();

            loop {
                let event = receiver.lock().unwrap().recv().unwrap();
                match event {
                    RecordEvent::Start => {
                        stream.play().unwrap();
                    }
                    RecordEvent::Stop => {
                        stream.pause().unwrap();
                    }
                }
            }
        });

        Self { thread, samples }
    }
}

pub struct Recorder {
    sender: mpsc::Sender<RecordEvent>,
    records: Vec<Record>,
}

impl Recorder {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel::<RecordEvent>();
        let receiver = Arc::new(Mutex::new(receiver));

        let host = cpal::default_host();

        // デフォルトの入力デバイスを取得
        let device = host
            .default_input_device()
            .ok_or("No default input device")
            .unwrap();

        // デバイスのデフォルト設定を取得
        let default_config = device.default_input_config().unwrap();

        let mut records = Vec::new();
        records.push(Record::new(receiver, device, default_config.config()));

        Self { sender, records }
    }

    pub fn start(&self) -> Result<(), String> {
        self.sender.send(RecordEvent::Start).unwrap();
        Ok(())
    }

    pub fn stop(&self) -> Result<(), String> {
        self.sender.send(RecordEvent::Stop).unwrap();
        Ok(())
    }
}
