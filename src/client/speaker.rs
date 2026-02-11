use esp_idf_hal::gpio::{Gpio1, Gpio2, Gpio4};
use esp_idf_hal::i2s::config::{Config, SlotMode, StdClkConfig, StdGpioConfig, StdSlotConfig};
use esp_idf_svc::hal::gpio::AnyIOPin;
use esp_idf_svc::hal::i2s::config::{DataBitWidth, StdConfig};
use esp_idf_svc::hal::i2s::{I2sDriver, I2sTx, I2S0};

pub struct Speaker {
    i2s_driver: I2sDriver<'static, I2sTx>,
}
impl Speaker {
    const SAMPLE_RATE: u32 = 16_000;

    pub fn new(i2s0: I2S0, bclk: Gpio1, dout: Gpio4, ws: Gpio2) -> anyhow::Result<Self> {
        log::info!("初始化扬声器...");

        // let std_config = StdConfig::philips(Self::SAMPLE_RATE, DataBitWidth::Bits16);
        let std_config = StdConfig::new(
            Config::default(),
            StdClkConfig::from_sample_rate_hz(Self::SAMPLE_RATE),
            StdSlotConfig::philips_slot_default(DataBitWidth::Bits16, SlotMode::Mono),
            StdGpioConfig::new(false, false, false),
        );
        let mclk = AnyIOPin::none();

        let mut i2s = I2sDriver::<I2sTx>::new_std_tx(i2s0, &std_config, bclk, dout, mclk, ws)?;
        i2s.tx_enable()?;

        log::info!("扬声器初始化完成");

        Ok(Self { i2s_driver: i2s })
    }

    /// 同步播放音频（阻塞）
    #[allow(unused)]
    pub fn play(&mut self, data: &[u8]) -> anyhow::Result<()> {
        self.i2s_driver.write_all(&data, 1000)?;
        Ok(())
    }

    pub fn play_chunked(&mut self, data: &[u8], chunk_size: usize) -> anyhow::Result<()> {
        let mut offset = 0;

        while offset < data.len() {
            let end = usize::min(offset + chunk_size, data.len());
            let chunk = &data[offset..end];
            self.i2s_driver.write(chunk, 1000)?;
            offset += chunk_size;
        }

        Ok(())
    }
}
