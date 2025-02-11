use crate::registers::{self, Register};

use heapless::Vec;

pub mod accel_gyro;
pub mod magnetometer;

pub use accel_gyro::AccelGyroConfig;
pub use magnetometer::MagnetometerConfig;

#[derive(Copy, Clone)]
pub enum InterruptPinMode {
    PushPull,
    OpenDrain,
}

#[derive(Copy, Clone)]
pub enum SPIMode {
    ThreeWire,
    FourWire,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub(crate) enum Endianness {
    BigEndian,
    LittleEndian,
}

pub struct DeviceConfig {
    reboot_memory_content: bool,
    block_data_update: bool,
    pub interrupt_active_low: bool,
    pub interrupt_pin_mode: InterruptPinMode,
    pub spi_mode: SPIMode,
    register_increment: bool,
    pub magnetometer: MagnetometerConfig,
    pub accel_gyro: AccelGyroConfig,
}

/// Device configuration.
impl Default for DeviceConfig {
    fn default() -> Self {
        Self {
            reboot_memory_content: false,
            block_data_update: false,
            interrupt_active_low: false,
            interrupt_pin_mode: InterruptPinMode::PushPull,
            spi_mode: SPIMode::FourWire,
            register_increment: true,
            magnetometer: Default::default(),
            accel_gyro: Default::default(),
        }
    }
}

impl DeviceConfig {
    pub fn all_registers(&self) -> Vec<(Register, u8), 16> {
        let mut all_registers = Vec::new();

        let ctrl_register_8 = registers::ctrl_reg8(
            self.reboot_memory_content,
            self.block_data_update,
            self.interrupt_active_low,
            self.interrupt_pin_mode,
            self.spi_mode,
            self.register_increment,
            self.accel_gyro.endianness,
            false,
        );

        all_registers
            .extend_from_slice(&[(Register::CTRL_REG8, ctrl_register_8)])
            .unwrap(); // +1 element -> 1 element total

        let ag_registers = self.accel_gyro.all_registers();
        all_registers.extend_from_slice(&ag_registers).unwrap(); // +7 elements -> 8 elements total

        let mag_registers = self.magnetometer.all_registers();
        all_registers.extend_from_slice(&mag_registers).unwrap(); // +5 elements -> 13 elements total

        all_registers
    }
}
