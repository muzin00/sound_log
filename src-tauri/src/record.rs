use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Record {
    samples: Arc<Mutex<Vec<f32>>>,
}

impl Record {
    pub fn new() -> Self {
        Self {
            samples: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn write(&self, samples: &[f32]) {
        for sample in samples {
            self.samples.lock().unwrap().push(*sample);
        }
    }
}
