#![cfg_attr(not(test), no_std)] // This allows us to use std in tests

/// Builder for an LSM9DS1 sensor.
pub mod builder;
/// Sensor configuration.
pub mod config;
/// Sensor interfaces.
pub mod interface;

mod registers;

#[cfg(test)]
mod tests;

pub use builder::Lsm9ds1Builder;
use config::DeviceConfig;
use core::error::Error;
use core::fmt::Display;
use embedded_hal::i2c;
use interface::Interface;
use registers::Register;

/// Driver Errors.
#[derive(Debug)]
pub enum Lsm9ds1Error {
    /// Error during I2C communication.
    I2cError(i2c::ErrorKind),
}

impl Display for Lsm9ds1Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for Lsm9ds1Error {}

/// An LSM9DS1 sensor.
pub struct Lsm9ds1<I: Interface> {
    interface: I,
    config: DeviceConfig,
}

impl<I: Interface> Lsm9ds1<I> {
    /// Initialize the device by applying all settings.
    pub fn init(&mut self) -> Result<(), Lsm9ds1Error> {
        let register_values = self.config.all_registers();
        for (reg, value) in register_values.into_iter() {
            self.interface.write(reg, value)?;
        }

        Ok(())
    }

    /// Enable or disable the accelerometer.
    pub fn set_accelerometer_enabled(&mut self, enabled: bool) -> Result<(), Lsm9ds1Error> {
        let ctrl_reg_5_xl = registers::ctrl_reg_5_xl(
            self.config.accel_gyro.accel_decimation,
            enabled,
            enabled,
            enabled,
        );

        self.interface
            .write(Register::CTRL_REG5_XL, ctrl_reg_5_xl)?;

        self.config.accel_gyro.accel_x_axis_enabled = enabled;
        self.config.accel_gyro.accel_y_axis_enabled = enabled;
        self.config.accel_gyro.accel_z_axis_enabled = enabled;

        Ok(())
    }

    /// Enable or disable the gyroscope.
    pub fn set_gyroscope_enabled(&mut self, enabled: bool) -> Result<(), Lsm9ds1Error> {
        let ctrl_reg_4 = registers::ctrl_reg_4(enabled, enabled, enabled);

        self.interface.write(Register::CTRL_REG4, ctrl_reg_4)?;

        self.config.accel_gyro.gyro_x_axis_enabled = enabled;
        self.config.accel_gyro.gyro_y_axis_enabled = enabled;
        self.config.accel_gyro.gyro_z_axis_enabled = enabled;

        Ok(())
    }

    /// Enable or disable the magnetometer.
    pub fn set_magnetometer_enabled(&mut self, enabled: bool) -> Result<(), Lsm9ds1Error> {
        let operating_mode = if enabled {
            config::magnetometer::OperatingMode::ContinuousConversion
        } else {
            config::magnetometer::OperatingMode::PowerDown
        };

        let ctrl_reg_3_m = registers::ctrl_rg_3_m(
            self.config.magnetometer.i2c_disabled,
            self.config.magnetometer.low_power_mode,
            self.config.magnetometer.spi_write_only,
            operating_mode,
        );

        self.interface.write(Register::CTRL_REG3_M, ctrl_reg_3_m)?;

        self.config.magnetometer.operating_mode = operating_mode;

        Ok(())
    }

    /// Set the sampling rate for the accelerometer and gyroscope.
    pub fn set_accel_gyro_sampling_rate(
        &mut self,
        rate: config::accel_gyro::AccelGyroSamplingRate,
    ) -> Result<(), Lsm9ds1Error> {
        let ctrl_reg_1_g = registers::ctrl_reg_1_g(rate, self.config.accel_gyro.gyro_full_scale);

        self.interface.write(Register::CTRL_REG1_G, ctrl_reg_1_g)?;

        self.config.accel_gyro.accel_gyro_sampling_rate = rate;

        Ok(())
    }

    /// Set the sampling rate for the accelerometer (if gyro is disabled).
    pub fn set_accel_sampling_rate(
        &mut self,
        rate: config::accel_gyro::AccelSamplingRate,
    ) -> Result<(), Lsm9ds1Error> {
        let ctrl_reg_6_xl = registers::ctrl_reg_6_xl(
            rate,
            self.config.accel_gyro.accel_full_scale,
            self.config.accel_gyro.accel_bandwidth,
        );

        self.interface
            .write(Register::CTRL_REG6_XL, ctrl_reg_6_xl)?;

        self.config.accel_gyro.accel_only_sampling_rate = rate;

        Ok(())
    }

    /// Read out gyroscope data in dps.
    pub fn get_gyroscope_data(&mut self) -> Result<(f32, f32, f32), Lsm9ds1Error> {
        let mut data = [0; 6];
        self.interface
            .read_multiple(Register::OUT_X_L_G, &mut data)?;

        let x_raw = (data[1] as i16) << 8 | data[0] as i16;
        let y_raw = (data[3] as i16) << 8 | data[2] as i16;
        let z_raw = (data[5] as i16) << 8 | data[4] as i16;

        let scale_factor = f32::from(self.config.accel_gyro.gyro_full_scale) / (i16::MAX as f32);

        let x_dps = x_raw as f32 * scale_factor;
        let y_dps = y_raw as f32 * scale_factor;
        let z_dps = z_raw as f32 * scale_factor;

        Ok((x_dps, y_dps, z_dps))
    }

    /// Read out accelerometer data in g.
    pub fn get_accelerometer_data(&mut self) -> Result<(f32, f32, f32), Lsm9ds1Error> {
        let mut data = [0; 6];
        self.interface
            .read_multiple(Register::OUT_X_L_XL, &mut data)?;

        let x_raw = (data[1] as i16) << 8 | data[0] as i16;
        let y_raw = (data[3] as i16) << 8 | data[2] as i16;
        let z_raw = (data[5] as i16) << 8 | data[4] as i16;

        let scale_factor = f32::from(self.config.accel_gyro.accel_full_scale) / (i16::MAX as f32);

        let x_g = x_raw as f32 * scale_factor;
        let y_g = y_raw as f32 * scale_factor;
        let z_g = z_raw as f32 * scale_factor;

        Ok((x_g, y_g, z_g))
    }

    /// Read out magnetometer data in Gauss.
    pub fn get_magnetometer_data(&mut self) -> Result<(f32, f32, f32), Lsm9ds1Error> {
        let mut data = [0; 6];
        self.interface
            .read_multiple(Register::OUT_X_L_M, &mut data)?;

        let x_raw = (data[1] as i16) << 8 | data[0] as i16;
        let y_raw = (data[3] as i16) << 8 | data[2] as i16;
        let z_raw = (data[5] as i16) << 8 | data[4] as i16;

        let scale_factor = f32::from(self.config.magnetometer.full_scale) / (i16::MAX as f32);

        let x_gauss = x_raw as f32 * scale_factor;
        let y_gauss = y_raw as f32 * scale_factor;
        let z_gauss = z_raw as f32 * scale_factor;

        Ok((x_gauss, y_gauss, z_gauss))
    }

    /// Read out chip identification for the accelerometer and gyroscope.
    pub fn who_am_i_ag(&mut self) -> Result<u8, Lsm9ds1Error> {
        self.interface.read(Register::WHO_AM_I)
    }

    /// Read out chip identification for the magnetometer.
    pub fn who_am_i_m(&mut self) -> Result<u8, Lsm9ds1Error> {
        self.interface.read(Register::WHO_AM_I_M)
    }

    /// Apply software reset.
    pub fn reset(&mut self) -> Result<(), Lsm9ds1Error> {
        let mut ctrl_reg = self.interface.read(Register::CTRL_REG8)?;
        ctrl_reg |= 0b1;
        self.interface.write(Register::CTRL_REG8, ctrl_reg)
    }

    /// Read out temperature in °C. Temperature is used for internal temperature compensation and
    /// not as a primary sensor output. Values will be inaccurate if primary sensor types aren't
    /// being sampled.
    pub fn temperature_c(&mut self) -> Result<f32, Lsm9ds1Error> {
        let temp_l = self.interface.read(Register::OUT_TEMP_L)?;
        let temp_h = self.interface.read(Register::OUT_TEMP_H)?;

        let temp: i16 = (temp_h as i16) << 8 | temp_l as i16;

        const BIAS: f32 = 25.0;
        const SCALE: f32 = 16.0;

        let temp = ((temp as f32) / SCALE) + BIAS;
        Ok(temp)
    }
}
