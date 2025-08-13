use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Record {
    pub channels: u16,
    pub sample_rate: u32,
    samples: Arc<Mutex<Vec<f32>>>,
}

impl Record {
    pub fn new() -> Self {
        Self {
            channels: 0,
            sample_rate: 0,
            samples: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn write(&self, samples: &[f32]) {
        for sample in samples {
            self.samples.lock().unwrap().push(*sample);
        }
    }

    pub fn read(&self) -> Vec<f32> {
        self.samples.lock().unwrap().clone()
    }
}
