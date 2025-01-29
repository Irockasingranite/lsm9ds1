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
    interface: I
}

impl<I: Interface> Lsm9ds1<I> {
    /// Create a new driver instance.
    pub fn new(interface: I) -> Self {
        Self {
            interface
        }
    }

    /// Read out chip identification
    pub fn who_am_i(&mut self) -> Result<u8, Error> {
        self.interface.read(Register::WHO_AM_I)
    }
}

