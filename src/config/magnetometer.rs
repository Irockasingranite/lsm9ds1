use super::Endianness;
use crate::registers::{self, Register};

#[derive(Copy, Clone)]
pub enum PerformanceMode {
    LowPower,
    MediumPerformance,
    HighPerformance,
    UtraHighPerformance,
}

#[derive(Copy, Clone)]
pub enum SamplingRate {
    _0p625Hz,
    _1p25Hz,
    _2p5Hz,
    _5Hz,
    _10Hz,
    _20Hz,
    _40Hz,
    _80Hz,
}

#[derive(Copy, Clone)]
pub enum FullScale {
    PlusMinus4Gauss,
    PlusMinus8Gauss,
    PlusMinus12Gauss,
    PlusMinus16Gauss,
}

#[derive(Copy, Clone)]
pub enum OperatingMode {
    ContinuousConversion,
    SingleConversion,
    PowerDown,
}

/// Magnetometer configuration.
pub struct MagnetometerConfig {
    /// Whether magnetometer component is enabled.
    pub enabled: bool,
    /// Enable internal temperature compensation.
    pub temperature_compensation: bool,
    /// Performance/power mode used for XY-axis measurement.
    pub xy_performance_mode: PerformanceMode,
    /// Performance/power mode used for Z-axis measurement.
    pub z_performance_mode: PerformanceMode,
    /// Sample rate.
    pub sampling_rate: SamplingRate,
    /// Enables sampling rates higher than 80Hz.
    pub fast_sampling: bool,
    /// Self test enable.
    pub self_test: bool,
    /// Data scale configuration. Determines the dynamic range of the sensor.
    pub full_scale: FullScale,
    /// Disable I²C interface.
    pub i2c_disabled: bool,
    /// Low power mode. If this is set to true, sampling rate is set to 0.625Hz and the system
    /// performs, for each channel, the minimum number of averages.
    pub low_power_mode: bool,
    /// Set SPI interface to write only.
    pub spi_write_only: bool,
    /// Operating mode.
    pub operating_mode: OperatingMode,
    /// Endianness of data returned in data registers.
    data_endianness: Endianness,
    /// Allows reading only high parts of data registers to increase reading efficiency.
    fast_read: bool,
    /// Block data updates until current data has been read.
    block_data_update: bool,
}

impl Default for MagnetometerConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            temperature_compensation: false,
            xy_performance_mode: PerformanceMode::LowPower,
            z_performance_mode: PerformanceMode::LowPower,
            sampling_rate: SamplingRate::_10Hz,
            fast_sampling: false,
            self_test: false,
            full_scale: FullScale::PlusMinus4Gauss,
            i2c_disabled: false,
            low_power_mode: false,
            spi_write_only: true,
            operating_mode: OperatingMode::PowerDown,
            data_endianness: Endianness::BigEndian,
            fast_read: false,
            block_data_update: false,
        }
    }
}

impl MagnetometerConfig {
    pub fn all_registers(&self) -> [(Register, u8); 5] {
        let ctrl_reg_1 = registers::ctrl_reg_1_m(
            self.temperature_compensation,
            self.xy_performance_mode,
            self.sampling_rate,
            self.fast_sampling,
            self.self_test,
        );
        let ctrl_reg_2 = registers::ctrl_reg_2_m(self.full_scale);
        let ctrl_reg_3 = registers::ctrl_rg_3_m(
            self.i2c_disabled,
            self.low_power_mode,
            self.spi_write_only,
            self.operating_mode,
        );
        let ctrl_reg_4 = registers::ctrl_reg_4_m(self.z_performance_mode, self.data_endianness);
        let ctrl_reg_5 = registers::ctrl_reg_5_m(self.fast_read, self.block_data_update);

        [
            (Register::CTRL_REG1_M, ctrl_reg_1),
            (Register::CTRL_REG2_M, ctrl_reg_2),
            (Register::CTRL_REG3_M, ctrl_reg_3),
            (Register::CTRL_REG4_M, ctrl_reg_4),
            (Register::CTRL_REG5_M, ctrl_reg_5),
        ]
    }
}
