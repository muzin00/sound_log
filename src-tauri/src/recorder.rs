use cpal::{
    self,
    traits::{DeviceTrait, HostTrait},
    Device, StreamConfig,
};

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
}
