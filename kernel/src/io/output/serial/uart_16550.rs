use bitflags::bitflags;
use uart_16550::{
    Config, Uart16550Tty, Uart16550TtyError,
    backend::{PioBackend, PortIoAddress},
};

#[repr(u16)]
// TODO: parse & validate via acpi alors scratch
pub enum SerialPortAddress {
    COM1 = 0x3F8,
    Com2 = 0x2F8,
    Com3 = 0x3E8,
    Com4 = 0x2E8,
}

bitflags! {
    pub struct SerialPortBitMask: u8 {
        const COM1 = 1 << 0;
        const COM2 = 1 << 1;
        const COM3 = 1 << 2;
        const COM4 = 1 << 3;
    }
}

impl From<SerialPortAddress> for u16 {
    fn from(value: SerialPortAddress) -> Self {
        value as u16
    }
}

pub struct Uart16550 {
    pub coms: [Option<Uart16550Tty<PioBackend>>; 4],
}

impl Uart16550 {
    /// # Safety
    ///
    /// Callers must ensure that the base port is valid and safe to use
    /// for the **whole lifetime** of the device(exclusively). Further, all
    /// [`NUM_REGISTERS`] registers must be safely reachable from the base
    /// address.
    ///
    /// [`NUM_REGISTERS`]: crate::spec::NUM_REGISTERS
    pub unsafe fn init(
        coms_enabled: SerialPortBitMask,
        mut config: [Option<Config>; 4],
    ) -> Result<Self, Uart16550TtyError<PortIoAddress>> {
        unsafe {
            Ok(Self {
                coms: [
                    if coms_enabled.contains(SerialPortBitMask::COM1) {
                        Some(Uart16550Tty::new_port(
                            SerialPortAddress::COM1.into(),
                            config[0].take().unwrap_or_default(),
                        )?)
                    } else {
                        None
                    },
                    if coms_enabled.contains(SerialPortBitMask::COM2) {
                        Some(Uart16550Tty::new_port(
                            SerialPortAddress::Com2.into(),
                            config[1].take().unwrap_or_default(),
                        )?)
                    } else {
                        None
                    },
                    if coms_enabled.contains(SerialPortBitMask::COM3) {
                        Some(Uart16550Tty::new_port(
                            SerialPortAddress::Com3.into(),
                            config[2].take().unwrap_or_default(),
                        )?)
                    } else {
                        None
                    },
                    if coms_enabled.contains(SerialPortBitMask::COM4) {
                        Some(Uart16550Tty::new_port(
                            SerialPortAddress::Com4.into(),
                            config[3].take().unwrap_or_default(),
                        )?)
                    } else {
                        None
                    },
                ],
            })
        }
    }
}
