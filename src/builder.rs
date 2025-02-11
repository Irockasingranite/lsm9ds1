use crate::config::{
    accel_gyro::{AccelGyroSamplingRate, AccelSamplingRate, GyroFullScale},
    magnetometer, DeviceConfig,
};
use crate::interface::Interface;
use crate::Error;
use crate::Lsm9ds1;

/// Builder for LSM9DS1 sensor.
#[derive(Default)]
pub struct Lsm9ds1Builder {
    config: DeviceConfig,
}

impl Lsm9ds1Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_config(&mut self, config: DeviceConfig) -> Self {
        Self { config }
    }

    pub fn init_on<I: Interface>(self, interface: I) -> Result<Lsm9ds1<I>, Error> {
        let mut device = Lsm9ds1::<I> {
            interface,
            config: self.config,
        };

        device.init()?;

        Ok(device)
    }

    pub fn with_accelerometer_enabled(mut self, enabled: bool) -> Self {
        self.config.accel_gyro.accel_x_axis_enabled = enabled;
        self.config.accel_gyro.accel_y_axis_enabled = enabled;
        self.config.accel_gyro.accel_z_axis_enabled = enabled;
        self
    }

    pub fn with_gyroscope_enabled(mut self, enabled: bool) -> Self {
        self.config.accel_gyro.gyro_x_axis_enabled = enabled;
        self.config.accel_gyro.gyro_y_axis_enabled = enabled;
        self.config.accel_gyro.gyro_z_axis_enabled = enabled;
        self
    }

    pub fn with_magnetometer_enabled(mut self, enabled: bool) -> Self {
        self.config.magnetometer.operating_mode = if enabled {
            magnetometer::OperatingMode::ContinuousConversion
        } else {
            magnetometer::OperatingMode::PowerDown
        };
        self
    }

    pub fn with_accel_gyro_sampling_rate(mut self, rate: AccelGyroSamplingRate) -> Self {
        self.config.accel_gyro.accel_gyro_sampling_rate = rate;
        self
    }

    pub fn with_accel_sampling_rate(mut self, rate: AccelSamplingRate) -> Self {
        self.config.accel_gyro.accel_only_sampling_rate = rate;
        self
    }

    pub fn with_magnetometer_sampling_rate(mut self, rate: magnetometer::SamplingRate) -> Self {
        self.config.magnetometer.sampling_rate = rate;
        self
    }

    pub fn with_accelerometer_scale(mut self, scale: magnetometer::FullScale) -> Self {
        self.config.magnetometer.full_scale = scale;
        self
    }

    pub fn with_gyroscope_scale(mut self, scale: GyroFullScale) -> Self {
        self.config.accel_gyro.gyro_full_scale = scale;
        self
    }

    pub fn with_magnetometer_scale(mut self, scale: magnetometer::FullScale) -> Self {
        self.config.magnetometer.full_scale = scale;
        self
    }
}
