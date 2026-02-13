use esp_idf_hal::gpio::{Gpio38, Gpio5, Gpio7};
use esp_idf_hal::i2s::config::{Config, SlotMode, StdClkConfig, StdGpioConfig, StdSlotConfig};
use esp_idf_svc::hal::gpio::AnyIOPin;
use esp_idf_svc::hal::i2s::config::{DataBitWidth, StdConfig};
use esp_idf_svc::hal::i2s::{I2sDriver, I2sRx, I2S1};

pub struct Mic {
    i2s_driver: I2sDriver<'static, I2sRx>,
    vad: VadDetector,
    status: MicStatus,
}

#[derive(Eq, PartialEq)]
enum MicStatus {
    /// 静音
    Silence,
    /// 说话中
    Speaking,
}

pub enum MicEvent {
    Start(Vec<u8>),
    End(Vec<u8>),
    Frame(Vec<u8>),
    Silence,
}

impl Mic {
    const BUF_SIZE: usize = 1024;
    const SAMPLE_RATE: u32 = 16_000;
    const VAD_THRESHOLD: f32 = 15000.;
    const MIN_SILENCE_FRAMES: usize = 32;
    pub fn new(i2s1: I2S1, bclk: Gpio5, sd: Gpio38, ws: Gpio7) -> anyhow::Result<Self> {
        log::info!("初始化麦克风...");

        let std_config = StdConfig::new(
            Config::default(),
            StdClkConfig::from_sample_rate_hz(Self::SAMPLE_RATE),
            StdSlotConfig::philips_slot_default(DataBitWidth::Bits16, SlotMode::Mono),
            StdGpioConfig::new(false, false, false),
        );

        let mclk = AnyIOPin::none();

        let mut i2s = I2sDriver::<I2sRx>::new_std_rx(i2s1, &std_config, bclk, sd, mclk, ws)?;
        i2s.rx_enable()?;

        log::info!("麦克风初始化完成");

        let vad = VadDetector::new(Self::VAD_THRESHOLD, Self::MIN_SILENCE_FRAMES);
        log::info!("初始化VAD完成");

        Ok(Self {
            i2s_driver: i2s,
            vad,
            status: MicStatus::Silence,
        })
    }

    pub fn read(&mut self) -> anyhow::Result<MicEvent> {
        let mut buffer = vec![0u8; Self::BUF_SIZE];

        let _ = self.i2s_driver.read(&mut buffer, 10);
        let is_speaking = self.vad.speeching(&buffer);
        // 说话中
        if is_speaking {
            // 当前状态为静音
            if self.status == MicStatus::Silence {
                self.status = MicStatus::Speaking;
                return Ok(MicEvent::Start(buffer));
            }
            return Ok(MicEvent::Frame(buffer));
        }

        match self.status {
            MicStatus::Silence => {
                // 静音
                Ok(MicEvent::Silence)
            }
            MicStatus::Speaking => {
                // 静音
                self.status = MicStatus::Silence;
                // 说话结束
                Ok(MicEvent::End(buffer))
            }
        }
    }
}

pub struct VadDetector {
    energy_threshold: f32,
    silence_counter: usize,
    min_silence_frames: usize,
    previous_energy: f32,
}

impl VadDetector {
    pub fn new(threshold: f32, min_silence_frames: usize) -> Self {
        Self {
            energy_threshold: threshold,
            silence_counter: 0,
            min_silence_frames,
            previous_energy: 0.0,
        }
    }

    pub fn speeching(&mut self, frame: &[u8]) -> bool {
        let energy = self.calculate_frame_energy(frame);
        if energy > self.energy_threshold {
            self.silence_counter = 0;
            true
        } else {
            self.silence_counter += 1;
            self.silence_counter <= self.min_silence_frames
        }
    }

    fn calculate_frame_energy(&mut self, frame: &[u8]) -> f32 {
        let emphasized = self.pre_emphasis(frame);
        let sum_of_squares: f32 = emphasized
            .iter()
            .map(|&sample| (sample as f32).powi(2))
            .sum();

        let current_energy = sum_of_squares / emphasized.len() as f32;
        let alpha = 0.1;
        self.previous_energy = self.previous_energy * (1.0 - alpha) + current_energy * alpha;
        self.previous_energy
    }

    fn pre_emphasis(&self, frame: &[u8]) -> Vec<i16> {
        let mut emphasized = Vec::with_capacity(frame.len());
        let alpha = 0.97; // 预加重系数
        emphasized.push((frame[0] as i16) - 128);
        for i in 1..frame.len() {
            let sample = (frame[i] as i16) - 128;
            let prev_sample = (frame[i - 1] as i16) - 128;
            emphasized.push(sample - (alpha * prev_sample as f32) as i16);
        }
        emphasized
    }
}
