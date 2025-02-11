use crate::config::{
    accel_gyro::{
        AccelBandWidth, AccelDecimation, AccelFullScale, AccelGyroSamplingRate, AccelLowpassCutoff,
        AccelSamplingRate, GyroFullScale,
    },
    magnetometer::{FullScale, OperatingMode, PerformanceMode, SamplingRate},
    Endianness, InterruptPinMode, SPIMode,
};

/// Register address of a sensor component.
#[derive(Clone, Copy)]
pub enum ComponentAddress {
    /// Accelerometer + gyroscope (also temperature).
    Ag(u8),
    /// Magnetometer.
    M(u8),
}

/// Device registers.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub enum Register {
    ACT_THS,
    ACT_DUR,
    INT_GEN_CFG_XL,
    INT_GEN_THS_X_XL,
    INT_GEN_THS_Y_XL,
    INT_GEN_THS_Z_XL,
    INT_GEN_DUR_XL,
    REFERENCE_G,
    INT1_CTRL,
    INT2_CTRL,
    WHO_AM_I,
    CTRL_REG1_G,
    CTRL_REG2_G,
    CTRL_REG3_G,
    ORIENT_CFG_G,
    INT_GEN_SRC_G,
    OUT_TEMP_L,
    OUT_TEMP_H,
    STATUS_REG_G,
    OUT_X_L_G,
    OUT_X_H_G,
    OUT_Y_L_G,
    OUT_Y_H_G,
    OUT_Z_L_G,
    OUT_Z_H_G,
    CTRL_REG4,
    CTRL_REG5_XL,
    CTRL_REG6_XL,
    CTRL_REG7_XL,
    CTRL_REG8,
    CTRL_REG9,
    CTRL_REG10,
    INT_GEN_SRC_XL,
    STATUS_REG_A,
    OUT_X_L_XL,
    OUT_X_H_XL,
    OUT_Y_L_XL,
    OUT_Y_H_XL,
    OUT_Z_L_XL,
    OUT_Z_H_XL,
    FIFO_CTRL,
    FIFO_SRC,
    INT_GEN_CFG_G,
    INT_GEN_THS_XH_G,
    INT_GEN_THS_XL_G,
    INT_GEN_THS_YH_G,
    INT_GEN_THS_YL_G,
    INT_GEN_THS_ZH_G,
    INT_GEN_THS_ZL_G,
    INT_GEN_DUR_G,
    OFFSET_X_REG_L_M,
    OFFSET_X_REG_H_M,
    OFFSET_Y_REG_L_M,
    OFFSET_Y_REG_H_M,
    OFFSET_Z_REG_L_M,
    OFFSET_Z_REG_H_M,
    WHO_AM_I_M,
    CTRL_REG1_M,
    CTRL_REG2_M,
    CTRL_REG3_M,
    CTRL_REG4_M,
    CTRL_REG5_M,
    STATUS_REG_M,
    OUT_X_L_M,
    OUT_X_H_M,
    OUT_Y_L_M,
    OUT_Y_H_M,
    OUT_Z_L_M,
    OUT_Z_H_M,
    INT_CFG_M,
    INT_SRC_M,
    INT_THS_L_M,
    INT_THS_H_M,
}

impl Register {
    /// Get sensor component the register belongs to, and the register address.
    pub fn addr(self) -> ComponentAddress {
        match self {
            Self::ACT_THS => ComponentAddress::Ag(0x04),
            Self::ACT_DUR => ComponentAddress::Ag(0x05),
            Self::INT_GEN_CFG_XL => ComponentAddress::Ag(0x06),
            Self::INT_GEN_THS_X_XL => ComponentAddress::Ag(0x07),
            Self::INT_GEN_THS_Y_XL => ComponentAddress::Ag(0x08),
            Self::INT_GEN_THS_Z_XL => ComponentAddress::Ag(0x09),
            Self::INT_GEN_DUR_XL => ComponentAddress::Ag(0x0a),
            Self::REFERENCE_G => ComponentAddress::Ag(0x0b),
            Self::INT1_CTRL => ComponentAddress::Ag(0x0c),
            Self::INT2_CTRL => ComponentAddress::Ag(0x0d),
            Self::WHO_AM_I => ComponentAddress::Ag(0x0f),
            Self::CTRL_REG1_G => ComponentAddress::Ag(0x10),
            Self::CTRL_REG2_G => ComponentAddress::Ag(0x11),
            Self::CTRL_REG3_G => ComponentAddress::Ag(0x12),
            Self::ORIENT_CFG_G => ComponentAddress::Ag(0x13),
            Self::INT_GEN_SRC_G => ComponentAddress::Ag(0x14),
            Self::OUT_TEMP_L => ComponentAddress::Ag(0x15),
            Self::OUT_TEMP_H => ComponentAddress::Ag(0x16),
            Self::STATUS_REG_G => ComponentAddress::Ag(0x17),
            Self::OUT_X_L_G => ComponentAddress::Ag(0x18),
            Self::OUT_X_H_G => ComponentAddress::Ag(0x19),
            Self::OUT_Y_L_G => ComponentAddress::Ag(0x1a),
            Self::OUT_Y_H_G => ComponentAddress::Ag(0x1b),
            Self::OUT_Z_L_G => ComponentAddress::Ag(0x1c),
            Self::OUT_Z_H_G => ComponentAddress::Ag(0x1d),
            Self::CTRL_REG4 => ComponentAddress::Ag(0x1e),
            Self::CTRL_REG5_XL => ComponentAddress::Ag(0xf1),
            Self::CTRL_REG6_XL => ComponentAddress::Ag(0x20),
            Self::CTRL_REG7_XL => ComponentAddress::Ag(0x21),
            Self::CTRL_REG8 => ComponentAddress::Ag(0x22),
            Self::CTRL_REG9 => ComponentAddress::Ag(0x23),
            Self::CTRL_REG10 => ComponentAddress::Ag(0x24),
            Self::INT_GEN_SRC_XL => ComponentAddress::Ag(0x26),
            Self::STATUS_REG_A => ComponentAddress::Ag(0x27),
            Self::OUT_X_L_XL => ComponentAddress::Ag(0x28),
            Self::OUT_X_H_XL => ComponentAddress::Ag(0x29),
            Self::OUT_Y_L_XL => ComponentAddress::Ag(0x2a),
            Self::OUT_Y_H_XL => ComponentAddress::Ag(0x2b),
            Self::OUT_Z_L_XL => ComponentAddress::Ag(0x2c),
            Self::OUT_Z_H_XL => ComponentAddress::Ag(0x2d),
            Self::FIFO_CTRL => ComponentAddress::Ag(0x2e),
            Self::FIFO_SRC => ComponentAddress::Ag(0x2f),
            Self::INT_GEN_CFG_G => ComponentAddress::Ag(0x30),
            Self::INT_GEN_THS_XH_G => ComponentAddress::Ag(0x31),
            Self::INT_GEN_THS_XL_G => ComponentAddress::Ag(0x32),
            Self::INT_GEN_THS_YH_G => ComponentAddress::Ag(0x33),
            Self::INT_GEN_THS_YL_G => ComponentAddress::Ag(0x34),
            Self::INT_GEN_THS_ZH_G => ComponentAddress::Ag(0x35),
            Self::INT_GEN_THS_ZL_G => ComponentAddress::Ag(0x36),
            Self::INT_GEN_DUR_G => ComponentAddress::Ag(0x37),
            Self::OFFSET_X_REG_L_M => ComponentAddress::M(0x05),
            Self::OFFSET_X_REG_H_M => ComponentAddress::M(0x06),
            Self::OFFSET_Y_REG_L_M => ComponentAddress::M(0x07),
            Self::OFFSET_Y_REG_H_M => ComponentAddress::M(0x08),
            Self::OFFSET_Z_REG_L_M => ComponentAddress::M(0x09),
            Self::OFFSET_Z_REG_H_M => ComponentAddress::M(0x0a),
            Self::WHO_AM_I_M => ComponentAddress::M(0x0f),
            Self::CTRL_REG1_M => ComponentAddress::M(0x20),
            Self::CTRL_REG2_M => ComponentAddress::M(0x21),
            Self::CTRL_REG3_M => ComponentAddress::M(0x22),
            Self::CTRL_REG4_M => ComponentAddress::M(0x23),
            Self::CTRL_REG5_M => ComponentAddress::M(0x24),
            Self::STATUS_REG_M => ComponentAddress::M(0x27),
            Self::OUT_X_L_M => ComponentAddress::M(0x28),
            Self::OUT_X_H_M => ComponentAddress::M(0x29),
            Self::OUT_Y_L_M => ComponentAddress::M(0x2a),
            Self::OUT_Y_H_M => ComponentAddress::M(0x2b),
            Self::OUT_Z_L_M => ComponentAddress::M(0x2c),
            Self::OUT_Z_H_M => ComponentAddress::M(0x2d),
            Self::INT_CFG_M => ComponentAddress::M(0x30),
            Self::INT_SRC_M => ComponentAddress::M(0x31),
            Self::INT_THS_L_M => ComponentAddress::M(0x32),
            Self::INT_THS_H_M => ComponentAddress::M(0x33),
        }
    }
}

pub fn ctrl_reg_1_g(
    accel_gyro_sampling_rate: AccelGyroSamplingRate,
    gyro_full_scale: GyroFullScale,
) -> u8 {
    let gyro_sampling_rate = (match accel_gyro_sampling_rate {
        AccelGyroSamplingRate::PowerDown => 0b000,
        AccelGyroSamplingRate::_14p9Hz => 0b001,
        AccelGyroSamplingRate::_59p5Hz => 0b010,
        AccelGyroSamplingRate::_119Hz => 0b011,
        AccelGyroSamplingRate::_238Hz => 0b100,
        AccelGyroSamplingRate::_476Hz => 0b101,
        AccelGyroSamplingRate::_952Hz => 0b110,
    }) << 5;

    let gyro_full_scale = (match gyro_full_scale {
        GyroFullScale::_245dps => 0b00,
        GyroFullScale::_500dps => 0b01,
        GyroFullScale::_2000dps => 0b11,
    }) << 3;

    let gyro_bandwidth = 0b00; // TODO

    gyro_sampling_rate | gyro_full_scale | gyro_bandwidth
}

pub fn ctrl_reg_2_g(gyro_high_pass_filter: bool, gyro_low_pass_filter: bool) -> u8 {
    match (gyro_high_pass_filter, gyro_low_pass_filter) {
        (false, false) => 0b00,
        (true, false) => 0b01,
        (_, true) => 0b10,
    }
}

pub fn ctrl_reg_3_g(gyro_low_power_mode: bool, gyro_high_pass_filter: bool) -> u8 {
    let gyro_high_pass_enable = (if gyro_high_pass_filter { 0b1 } else { 0b0 }) << 6;
    let gyro_low_power = (if gyro_low_power_mode { 0b1 } else { 0b0 }) << 7;
    let gyro_hpf_cutoff = 0b0; // TODO

    gyro_low_power | gyro_high_pass_enable | gyro_hpf_cutoff
}

pub fn ctrl_reg_4(
    gyro_x_axis_enabled: bool,
    gyro_y_axis_enabled: bool,
    gyro_z_axis_enabled: bool,
) -> u8 {
    let gyro_x_en = (if gyro_x_axis_enabled { 0b1 } else { 0b0 }) << 3;
    let gyro_y_en = (if gyro_y_axis_enabled { 0b1 } else { 0b0 }) << 4;
    let gyro_z_en = (if gyro_z_axis_enabled { 0b1 } else { 0b0 }) << 5;

    gyro_x_en | gyro_y_en | gyro_z_en
}

pub fn ctrl_reg_5_xl(
    accel_decimation: AccelDecimation,
    accel_x_axis_enabled: bool,
    accel_y_axis_enabled: bool,
    accel_z_axis_enabled: bool,
) -> u8 {
    let accel_dec = (match accel_decimation {
        AccelDecimation::None => 0b00,
        AccelDecimation::Half => 0b01,
        AccelDecimation::Quarter => 0b10,
        AccelDecimation::Eighth => 0b11,
    }) << 6;

    let accel_x_en = (if accel_x_axis_enabled { 0b1 } else { 0b0 }) << 3;
    let accel_y_en = (if accel_y_axis_enabled { 0b1 } else { 0b0 }) << 4;
    let accel_z_en = (if accel_z_axis_enabled { 0b0 } else { 0b1 }) << 5;

    accel_dec | accel_x_en | accel_y_en | accel_z_en
}

pub fn ctrl_reg_6_xl(
    accel_only_sampling_rate: AccelSamplingRate,
    accel_full_scale: AccelFullScale,
    accel_bandwidth: AccelBandWidth,
) -> u8 {
    let accel_sampling_rate = (match accel_only_sampling_rate {
        AccelSamplingRate::PowerDown => 0b000,
        AccelSamplingRate::_10Hz => 0b001,
        AccelSamplingRate::_50Hz => 0b010,
        AccelSamplingRate::_119Hz => 0b011,
        AccelSamplingRate::_238Hz => 0b100,
        AccelSamplingRate::_476Hz => 0b101,
        AccelSamplingRate::_952Hz => 0b110,
    }) << 5;

    let accel_full_scale = (match accel_full_scale {
        AccelFullScale::PlusMinus2g => 0b00,
        AccelFullScale::PlusMinus4g => 0b10,
        AccelFullScale::PlutMinus8g => 0b11,
        AccelFullScale::PlusMinus16g => 0b01,
    }) << 3;

    let accel_bandwidth = match accel_bandwidth {
        AccelBandWidth::Auto => 0b000,
        AccelBandWidth::_408Hz => 0b100,
        AccelBandWidth::_211Hz => 0b101,
        AccelBandWidth::_105Hz => 0b110,
        AccelBandWidth::_50Hz => 0b111,
    }; // << 0

    accel_sampling_rate | accel_full_scale | accel_bandwidth
}

pub fn ctrl_reg_7_xl(
    accel_high_resolution: bool,
    accel_low_pass_cutoff: AccelLowpassCutoff,
    accel_filter_enabled: bool,
) -> u8 {
    let accel_high_res = (if accel_high_resolution { 0b1 } else { 0b0 }) << 7;

    let accel_low_pass_cutoff = (match accel_low_pass_cutoff {
        AccelLowpassCutoff::_ODRby9 => 0b10,
        AccelLowpassCutoff::_ODRBy50 => 0b00,
        AccelLowpassCutoff::_ODRBy100 => 0b01,
        AccelLowpassCutoff::_ODRBy400 => 0b11,
    }) << 5;

    let accel_filter_enabled = (if accel_filter_enabled { 0b1 } else { 0b0 }) << 2;

    accel_high_res | accel_low_pass_cutoff | accel_filter_enabled
}

pub fn ctrl_reg8(
    reboot_memory_content: bool,
    block_data_update: bool,
    interrupt_active_low: bool,
    interrupt_pin_mode: InterruptPinMode,
    spi_mode: SPIMode,
    register_increment: bool,
    accel_gyro_endianness: Endianness,
    reset: bool,
) -> u8 {
    let reboot_memory_content = (if reboot_memory_content { 0b1 } else { 0b0 }) << 7;
    let block_data_update = (if block_data_update { 0b1 } else { 0b0 }) << 6;
    let interrupt_active_low = (if interrupt_active_low { 0b1 } else { 0b0 }) << 5;
    let interrupt_pin_mode = (match interrupt_pin_mode {
        InterruptPinMode::PushPull => 0b0,
        InterruptPinMode::OpenDrain => 0b1,
    }) << 4;
    let spi_mode = (match spi_mode {
        SPIMode::FourWire => 0b0,
        SPIMode::ThreeWire => 0b1,
    }) << 3;

    let register_increment = (if register_increment { 0b1 } else { 0b0 }) << 2;

    let ag_endianness = (match accel_gyro_endianness {
        Endianness::BigEndian => 0b0,
        Endianness::LittleEndian => 0b1,
    }) << 1;

    let reset = if reset { 0b1 } else { 0b0 }; // << 0

    reboot_memory_content
        | block_data_update
        | interrupt_active_low
        | interrupt_pin_mode
        | spi_mode
        | register_increment
        | ag_endianness
        | reset
}

pub fn ctrl_reg_1_m(
    temperature_compensation: bool,
    xy_performance_mode: PerformanceMode,
    sampling_rate: SamplingRate,
    fast_sampling: bool,
    self_test: bool,
) -> u8 {
    let temperature_compensation = (if temperature_compensation { 1 } else { 0 }) << 7;
    let xy_performance_mode = (match xy_performance_mode {
        PerformanceMode::LowPower => 0b00,
        PerformanceMode::MediumPerformance => 0b01,
        PerformanceMode::HighPerformance => 0b10,
        PerformanceMode::UtraHighPerformance => 0b11,
    }) << 5;
    let sampling_rate = (match sampling_rate {
        SamplingRate::_0p625Hz => 0b000,
        SamplingRate::_1p25Hz => 0b001,
        SamplingRate::_2p5Hz => 0b010,
        SamplingRate::_5Hz => 0b011,
        SamplingRate::_10Hz => 0b100,
        SamplingRate::_20Hz => 0b101,
        SamplingRate::_40Hz => 0b110,
        SamplingRate::_80Hz => 0b111,
    }) << 2;
    let fast_odr = (if fast_sampling { 1 } else { 0 }) << 1;
    let self_test = if self_test { 1 } else { 0 }; // << 0

    temperature_compensation | xy_performance_mode | sampling_rate | fast_odr | self_test
}

pub fn ctrl_reg_2_m(full_scale: FullScale) -> u8 {
    (match full_scale {
        FullScale::PlusMinus4Gauss => 0b00,
        FullScale::PlusMinus8Gauss => 0b01,
        FullScale::PlusMinus12Gauss => 0b10,
        FullScale::PlusMinus16Gauss => 0b11,
    }) << 5
}

pub fn ctrl_rg_3_m(
    i2c_disabled: bool,
    low_power_mode: bool,
    spi_write_only: bool,
    operating_mode: OperatingMode,
) -> u8 {
    let i2c_disabled = (if i2c_disabled { 1 } else { 0 }) << 7;
    let low_power_mode = (if low_power_mode { 1 } else { 0 }) << 5;
    let spi_mode = (if spi_write_only { 0 } else { 1 }) << 2;
    let operating_mode = match operating_mode {
        OperatingMode::ContinuousConversion => 0b00,
        OperatingMode::SingleConversion => 0b01,
        OperatingMode::PowerDown => 0b10,
    }; // << 0

    i2c_disabled | low_power_mode | spi_mode | operating_mode
}

pub fn ctrl_reg_4_m(z_performance_mode: PerformanceMode, data_endianness: Endianness) -> u8 {
    let z_performance_mode = (match z_performance_mode {
        PerformanceMode::LowPower => 0b00,
        PerformanceMode::MediumPerformance => 0b01,
        PerformanceMode::HighPerformance => 0b10,
        PerformanceMode::UtraHighPerformance => 0b11,
    }) << 2;
    let endianness = (match data_endianness {
        Endianness::BigEndian => 0b0,
        Endianness::LittleEndian => 0b1,
    }) << 1;

    z_performance_mode | endianness
}

pub fn ctrl_reg_5_m(fast_read: bool, block_data_update: bool) -> u8 {
    let fast_read = (if fast_read { 1 } else { 0 }) << 7;
    let block_data_update = (if block_data_update { 1 } else { 0 }) << 6;

    fast_read | block_data_update
}
