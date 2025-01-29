/// Sensor Components
pub enum Component {
    /// Accelerometer + gyroscope (also temperature).
    Ag,
    /// Magnetometer.
    M,
}

#[allow(non_camel_case_types)]
pub enum Register {
    WHO_AM_I,
    // TODO
}

impl Register {
    pub fn addr(self) -> (Component, u8) {
        match self {
            Self::WHO_AM_I => (Component::Ag, 0x0f),
        }
    }
}
