pub enum SerialPortAddress {
    Com1 = 0x3F8,
    Com2 = 0x2F8,
    Com3 = 0x3E8,
    Com4 = 0x2E8,
}

#[derive(Default)]
pub struct Uart16550 {}

impl Uart16550 {
    pub fn new() -> Self {
        Self {}
    }
}
