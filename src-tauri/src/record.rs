pub struct Record {
    samples: Vec<f32>,
}

impl Record {
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
        }
    }

    pub fn write(&mut self, samples: &[f32]) {
        for sample in samples {
            self.samples.push(*sample);
        }
    }
}
