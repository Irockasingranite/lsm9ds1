use super::Endianness;
use crate::registers::{self, Register};

#[derive(Copy, Clone)]
pub enum AccelGyroSamplingRate {
    PowerDown,
    _14p9Hz,
    _59p5Hz,
    _119Hz,
    _238Hz,
    _476Hz,
    _952Hz,
}

#[derive(Copy, Clone)]
pub enum AccelSamplingRate {
    PowerDown,
    _10Hz,
    _50Hz,
    _119Hz,
    _238Hz,
    _476Hz,
    _952Hz,
}

// TODO: Gyro bandwidth selection
// TODO: High pass cutoff

#[derive(Copy, Clone)]
pub enum GyroFullScale {
    _245dps,
    _500dps,
    _2000dps,
}

impl From<GyroFullScale> for f32 {
    fn from(gyro_full_scale: GyroFullScale) -> f32 {
        match gyro_full_scale {
            GyroFullScale::_245dps => 245.0,
            GyroFullScale::_500dps => 500.0,
            GyroFullScale::_2000dps => 2000.0,
        }
    }
}

#[derive(Copy, Clone)]
pub enum AccelFullScale {
    PlusMinus2g,
    PlusMinus4g,
    PlutMinus8g,
    PlusMinus16g,
}

impl From<AccelFullScale> for f32 {
    fn from(accel_full_scale: AccelFullScale) -> f32 {
        match accel_full_scale {
            AccelFullScale::PlusMinus2g => 2.0,
            AccelFullScale::PlusMinus4g => 4.0,
            AccelFullScale::PlutMinus8g => 8.0,
            AccelFullScale::PlusMinus16g => 16.0,
        }
    }
}

#[derive(Copy, Clone)]
pub enum AccelDecimation {
    None,
    Half,
    Quarter,
    Eighth,
}

#[derive(Copy, Clone)]
pub enum AccelBandWidth {
    Auto,
    _408Hz,
    _211Hz,
    _105Hz,
    _50Hz,
}

#[derive(Copy, Clone)]
pub enum AccelLowpassCutoff {
    _ODRby9,
    _ODRBy50,
    _ODRBy100,
    _ODRBy400,
}

/// Configuration of Accelerometer/Gyroscope component.
pub struct AccelGyroConfig {
    /// Sampling rate if both accelerometer and gyroscope are active.
    pub accel_gyro_sampling_rate: AccelGyroSamplingRate,
    /// Sampling rate of only accelerometer is active.
    pub accel_only_sampling_rate: AccelSamplingRate,
    /// Data scale of gyroscope. Determines the dynamic range.
    pub gyro_full_scale: GyroFullScale,
    /// Apply high pass filter to gyroscope data
    pub gyro_high_pass_filter: bool,
    /// Apply low pass filter to gyroscope data.
    pub gyro_low_pass_filter: bool,
    /// Enable low power mode.
    pub gyro_low_power_mode: bool,
    /// Enable X-axis gyroscope data.
    pub gyro_x_axis_enabled: bool,
    /// Enable Y-Axis gyroscope data.
    pub gyro_y_axis_enabled: bool,
    /// Enable Y-Axis gyroscope data.
    pub gyro_z_axis_enabled: bool,
    /// Decimation of accelerometer data.
    pub accel_decimation: AccelDecimation,
    /// Enable X-Axis accelerometer data.
    pub accel_x_axis_enabled: bool,
    /// Enable Y-Axis accelerometer data.
    pub accel_y_axis_enabled: bool,
    /// Enable Z-Axis accelerometer data.
    pub accel_z_axis_enabled: bool,
    /// Data scale of accelerometer. Determines the dynamic range.
    pub accel_full_scale: AccelFullScale,
    /// Accelerometer bandwidth.
    pub accel_bandwidth: AccelBandWidth,
    /// Accelerometer high resolution mode.
    pub accel_high_resolution: bool,
    /// Accelerometer low pass cutoff frequency.
    pub accel_low_pass_cutoff: AccelLowpassCutoff,
    /// Enable internal digital filter (high pass and low pass) for accelerometer.
    pub accel_filter_enabled: bool,
    /// Endianness data output
    pub(super) endianness: Endianness,
}

impl Default for AccelGyroConfig {
    fn default() -> Self {
        Self {
            accel_gyro_sampling_rate: AccelGyroSamplingRate::_14p9Hz,
            accel_only_sampling_rate: AccelSamplingRate::_10Hz,
            gyro_full_scale: GyroFullScale::_245dps,
            gyro_high_pass_filter: false,
            gyro_low_pass_filter: false,
            gyro_low_power_mode: false,
            gyro_x_axis_enabled: true,
            gyro_y_axis_enabled: true,
            gyro_z_axis_enabled: true,
            accel_decimation: AccelDecimation::None,
            accel_x_axis_enabled: true,
            accel_y_axis_enabled: true,
            accel_z_axis_enabled: true,
            accel_full_scale: AccelFullScale::PlusMinus2g,
            accel_bandwidth: AccelBandWidth::Auto,
            accel_high_resolution: false,
            accel_low_pass_cutoff: AccelLowpassCutoff::_ODRBy50,
            accel_filter_enabled: false,
            endianness: Endianness::BigEndian,
        }
    }
}

impl AccelGyroConfig {
    pub fn all_registers(&self) -> [(Register, u8); 7] {
        let ctrl_reg_1_g =
            registers::ctrl_reg_1_g(self.accel_gyro_sampling_rate, self.gyro_full_scale);
        let ctrl_reg_2_g =
            registers::ctrl_reg_2_g(self.gyro_high_pass_filter, self.gyro_low_pass_filter);
        let ctrl_reg_3_g =
            registers::ctrl_reg_3_g(self.gyro_low_power_mode, self.gyro_high_pass_filter);
        let ctrl_reg_4 = registers::ctrl_reg_4(
            self.gyro_x_axis_enabled,
            self.gyro_y_axis_enabled,
            self.gyro_z_axis_enabled,
        );
        let ctrl_reg_5_xl = registers::ctrl_reg_5_xl(
            self.accel_decimation,
            self.accel_x_axis_enabled,
            self.accel_y_axis_enabled,
            self.accel_z_axis_enabled,
        );
        let ctrl_reg_6_xl = registers::ctrl_reg_6_xl(
            self.accel_only_sampling_rate,
            self.accel_full_scale,
            self.accel_bandwidth,
        );
        let ctrl_reg_7_xl = registers::ctrl_reg_7_xl(
            self.accel_high_resolution,
            self.accel_low_pass_cutoff,
            self.accel_filter_enabled,
        );

        [
            (Register::CTRL_REG1_G, ctrl_reg_1_g),
            (Register::CTRL_REG2_G, ctrl_reg_2_g),
            (Register::CTRL_REG3_G, ctrl_reg_3_g),
            (Register::CTRL_REG4, ctrl_reg_4),
            (Register::CTRL_REG5_XL, ctrl_reg_5_xl),
            (Register::CTRL_REG6_XL, ctrl_reg_6_xl),
            (Register::CTRL_REG7_XL, ctrl_reg_7_xl),
        ]
    }
}
