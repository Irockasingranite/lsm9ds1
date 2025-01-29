use embedded_hal::i2c::{self, ErrorKind, ErrorType, I2c, Operation, SevenBitAddress};

/// A dummy I2C bus that sends preset data and records what is sent to it.
pub struct DummyI2c {
    last_addr: SevenBitAddress,
    rx_buffer: Vec<u8>,
    tx_buffer: Vec<u8>,
}

/// Errors reported by the dummy I2C bus.
#[derive(Debug)]
pub enum DummyError {}

impl i2c::Error for DummyError {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

impl DummyI2c {
    /// Create a new dummy bus.
    pub fn new() -> Self {
        Self {
            last_addr: 0x00,
            rx_buffer: Vec::new(),
            tx_buffer: vec![0u8],
        }
    }

    /// Create a new dummy bus with preset tx data.
    pub fn new_with_tx(tx: &[u8]) -> Self {
        let mut i2c = Self::new();
        i2c.set_tx(tx);
        i2c
    }

    /// Last message received by the bus.
    pub fn rx(&self) -> Vec<u8> {
        self.rx_buffer.clone()
    }

    /// Set data to be sent by the bus.
    pub fn set_tx(&mut self, bytes: &[u8]) {
        self.tx_buffer = bytes.to_owned();
    }

    /// Last address accessed via the bus.
    pub fn last_addr(&self) -> u8 {
        self.last_addr
    }
}

impl ErrorType for DummyI2c {
    type Error = DummyError;
}

impl I2c<SevenBitAddress> for DummyI2c {
    fn transaction(
        &mut self,
        address: SevenBitAddress,
        ops: &mut [Operation],
    ) -> Result<(), Self::Error> {
        self.last_addr = address;

        for op in ops.iter_mut() {
            match op {
                Operation::Read(buf) => {
                    let tx_iter = self.tx_buffer.iter().cycle();
                    for (buf_byte, tx_byte) in buf.iter_mut().zip(tx_iter) {
                        *buf_byte = *tx_byte;
                    }
                }
                Operation::Write(buf) => {
                    self.rx_buffer = Vec::from(*buf);
                }
            }
        }

        Ok(())
    }
}

#[test]
fn write_to_dummy_i2c() {
    let mut i2c = DummyI2c::new();

    let addr: u8 = 0xaa;
    let msg = vec![0xbb, 0xcc];
    i2c.write(addr, &msg).expect("Error during I2C write");

    assert_eq!(
        i2c.last_addr(),
        addr,
        "Address received by dummy doesn't match"
    );

    let rx = i2c.rx();
    assert_eq!(msg, rx, "Data received by dummy doesn't match sent message");
}

#[test]
fn read_from_dummy_i2c() {
    let mut i2c = DummyI2c::new_with_tx(&[1, 2, 3]);

    let addr = 0xbb;
    let mut buf = vec![0, 0, 0, 0, 0];

    i2c.read(addr, &mut buf).expect("Error during I2C read");

    assert_eq!(
        i2c.last_addr(),
        addr,
        "Address received by dummy doesn't match"
    );

    assert_eq!(
        buf,
        vec![1, 2, 3, 1, 2],
        "Data read from dummy doesn't match set TX buffer"
    );
}

#[test]
fn read_write_dummy_i2c() {
    let mut i2c = DummyI2c::new_with_tx(&[1, 1, 2, 3, 5, 8, 13]);

    let addr = 0xcc;
    let tx = vec![100, 99, 98];
    let mut rx = vec![0, 0, 0, 0, 0];

    i2c.write_read(addr, &tx, &mut rx)
        .expect("Error during I2C write_read");

    assert_eq!(
        i2c.last_addr(),
        addr,
        "Address reveived by dummy doesn't match"
    );
    assert_eq!(i2c.rx(), tx, "Data received by dummy doesn't match");
    assert_eq!(
        rx,
        vec![1, 1, 2, 3, 5],
        "Data read from dummy doesn't match"
    );
}
