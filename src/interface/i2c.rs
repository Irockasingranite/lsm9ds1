use core::slice;

use embedded_hal::i2c::Error as _;
use embedded_hal::i2c::I2c;

use crate::interface::Interface;
use crate::registers::{Component, Register};
use crate::Error;

/// I2C address of the Accelerometer/Gyroscope component.
pub enum AddressAg {
    /// Address is 0x6a.
    _0x6a,
    /// Address is 0x6b.
    _0x6b,
}

/// I2c address of the Magnetometer component.
pub enum AddressM {
    /// Address is 0x1c.
    _0x1c,
    /// Address is 0x1e.
    _0x1e,
}

trait Address {
    fn addr(&self) -> u8;
}

impl Address for AddressAg {
    fn addr(&self) -> u8 {
        match self {
            Self::_0x6a => 0x6a,
            Self::_0x6b => 0x6b,
        }
    }
}

impl Address for AddressM {
    fn addr(&self) -> u8 {
        match self {
            Self::_0x1c => 0x1c,
            Self::_0x1e => 0x1e,
        }
    }
}

/// Configuration for an I2C interface.
pub struct Config {
    /// Address of the Accelerometer/Gyroscope component.
    pub addr_ag: AddressAg,
    /// Address of the Magnetometer component.
    pub addr_m: AddressM,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            addr_ag: AddressAg::_0x6b,
            addr_m: AddressM::_0x1e,
        }
    }
}

/// I2C Interface to sensor.
pub struct I2cInterface<I2C> {
    /// I2C bus.
    bus: I2C,
    /// Interface Configuration.
    config: Config,
}

impl<I2C: I2c> I2cInterface<I2C> {
    /// Create a new I2C interface.
    pub fn new(i2c: I2C, config: Config) -> Self {
        Self { bus: i2c, config }
    }

    /// Utility function used by tests to inspect bus state.
    #[cfg(test)]
    pub fn bus(&self) -> &I2C {
        &self.bus
    }
}

impl<I2C: I2c> Interface for I2cInterface<I2C> {
    fn read(&mut self, reg: Register) -> Result<u8, Error> {
        let (device, reg_addr) = reg.addr();
        let device_addr = match device {
            Component::Ag => self.config.addr_ag.addr(),
            Component::M => self.config.addr_m.addr(),
        };
        let mut buf: u8 = 0;

        self.bus
            .write_read(device_addr, &[reg_addr], slice::from_mut(&mut buf))
            .map(|()| buf)
            .map_err(|e| Error::I2cError(e.kind()))
    }

    fn write(&mut self, reg: Register, value: u8) -> Result<(), Error> {
        let (device, reg_addr) = reg.addr();
        let device_addr = match device {
            Component::Ag => self.config.addr_ag.addr(),
            Component::M => self.config.addr_m.addr(),
        };

        self.bus
            .write(device_addr, &[reg_addr, value])
            .map_err(|e| Error::I2cError(e.kind()))
    }
}

#[cfg(test)]
use crate::tests::dummy_i2c::DummyI2c;

#[test]
fn read_reg_ag() {
    let i2c = DummyI2c::new();
    let cfg = Config {
        addr_ag: AddressAg::_0x6b,
        addr_m: AddressM::_0x1e,
    };

    let mut interface = I2cInterface::new(i2c, cfg);

    let _ = interface
        .read(Register::WHO_AM_I)
        .expect("Error in I2C interface");

    assert_eq!(interface.bus().last_addr(), 0x6b, "Wrong I2C address");
    assert_eq!(interface.bus().rx(), vec![0x0f], "Wrong register address");
}

#[test]
fn write_reg_ag() {
    let i2c = DummyI2c::new();
    let cfg = Config {
        addr_ag: AddressAg::_0x6a,
        addr_m: AddressM::_0x1c,
    };

    let mut interface = I2cInterface::new(i2c, cfg);

    interface
        .write(Register::WHO_AM_I, 0xff)
        .expect("Error in I2C interface");

    assert_eq!(interface.bus().last_addr(), 0x6a, "Wrong I2C address");
    assert_eq!(
        interface.bus().rx(),
        vec![0x0f, 0xff],
        "Wrong register and/or data"
    );
}

// TODO: Tests for Magnetometer registers
