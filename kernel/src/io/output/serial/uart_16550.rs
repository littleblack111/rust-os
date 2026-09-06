use uart_16550::{
    Config, Uart16550Tty, Uart16550TtyError,
    backend::{PioBackend, PortIoAddress},
};

#[repr(u16)]
// TODO: parse & validate via acpi alors scratch
enum SerialPortAddress {
    COM1 = 0x3F8,
    COM2 = 0x2F8,
    COM3 = 0x3E8,
    COM4 = 0x2E8,
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
        // TODO: more type safe
        mut config: [Option<Config>; 4],
    ) -> Result<Self, Uart16550TtyError<PortIoAddress>> {
        unsafe {
            Ok(Self {
                coms: [
                    init_port(&mut config[0], SerialPortAddress::COM1)?,
                    init_port(&mut config[1], SerialPortAddress::COM2)?,
                    init_port(&mut config[2], SerialPortAddress::COM3)?,
                    init_port(&mut config[3], SerialPortAddress::COM4)?,
                ],
            })
        }
    }
}

#[inline(always)]
unsafe fn init_port(
    cfg: &mut Option<Config>,
    addr: SerialPortAddress,
) -> Result<Option<Uart16550Tty<PioBackend>>, Uart16550TtyError<PortIoAddress>> {
    cfg.take().map(|c| unsafe { Uart16550Tty::new_port(addr.into(), c) }).transpose()
}
