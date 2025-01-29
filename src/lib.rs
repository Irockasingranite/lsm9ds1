#![cfg_attr(not(test), no_std)] // This allows us to use std in tests

/// Sensor interfaces.
pub mod interface;
mod registers;

#[cfg(test)]
mod tests;

use embedded_hal::i2c;
use interface::Interface;
use registers::Register;

/// Driver Errors.
#[derive(Debug)]
pub enum Error {
    /// Error during I2C communication.
    I2cError(i2c::ErrorKind),
}

/// An LSM9DS1 sensor.
pub struct Lsm9ds1<I: Interface> {
    interface: I,
}

impl<I: Interface> Lsm9ds1<I> {
    /// Create a new driver instance.
    pub fn new(interface: I) -> Self {
        Self { interface }
    }

    /// Read out chip identification for the accelerometer and gyroscope.
    pub fn who_am_i_ag(&mut self) -> Result<u8, Error> {
        self.interface.read(Register::WHO_AM_I)
    }

    /// Read out chip identification for the magnetometer.
    pub fn who_am_i_m(&mut self) -> Result<u8, Error> {
        self.interface.read(Register::WHO_AM_I_M)
    }

    pub fn reset(&mut self) -> Result<(), Error> {
        let mut ctrl_reg = self.interface.read(Register::CTRL_REG8)?;
        ctrl_reg |= 0b1;
        self.interface.write(Register::CTRL_REG8, ctrl_reg)
    }

    /// Read out temperature in °C. Temperature is used for internal temperature compensation and
    /// not as a primary sensor output. Values will be inaccurate if primary sensor types aren't
    /// being sampled.
    pub fn temperature_c(&mut self) -> Result<f32, Error> {
        let temp_l = self.interface.read(Register::OUT_TEMP_L)?;
        let temp_h = self.interface.read(Register::OUT_TEMP_H)?;

        let temp: i16 = (temp_h as i16) << 8 | temp_l as i16;

        const BIAS: f32 = 25.0;
        const SCALE: f32 = 16.0;

        let temp = ((temp as f32) / SCALE) + BIAS;
        Ok(temp)
    }
}
