use crate::registers::Register;
use crate::Error;

/// An interface to the sensor.
pub trait Interface {
    /// Write a value to a register.
    fn write(&mut self, reg: Register, value: u8) -> Result<(), Error>;

    /// Read a value from a register.
    fn read(&mut self, reg: Register) -> Result<u8, Error>;
}

pub mod i2c;
