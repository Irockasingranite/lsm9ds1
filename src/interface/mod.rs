use crate::registers::Register;
use crate::Lsm9ds1Error;

pub mod i2c;

/// An interface to the sensor.
pub trait Interface {
    /// Write a value to a register.
    fn write(&mut self, reg: Register, value: u8) -> Result<(), Lsm9ds1Error>;

    /// Read a value from a register.
    fn read(&mut self, reg: Register) -> Result<u8, Lsm9ds1Error>;

    /// Read multiple values from registers in sequence.
    fn read_multiple(&mut self, start_reg: Register, buffer: &mut [u8]) -> Result<(), Lsm9ds1Error>;
}

pub use i2c::I2cInterface;
