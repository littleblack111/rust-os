use spin::{lazylock::LazyLock, mutex::Mutex};

use crate::io::output::serial::uart_16550::{SerialPortBitMask, Uart16550};

// TODO: use linkme instead
pub static EARLY_PRINTK_SERIAL: LazyLock<Mutex<Uart16550>> = LazyLock::new(|| {
    Mutex::new(unsafe { Uart16550::init(SerialPortBitMask::COM1, [const { None }; 4]).unwrap() })
});

#[macro_export]
macro_rules! printk {
	($($arg:tt)*) => ({
		use ::core::fmt::Write;
		write!($crate::init::earlyprintk::EARLY_PRINTK_SERIAL.lock().coms[0].as_mut().unwrap(), $($arg)*)
	});
}

#[macro_export]
macro_rules! printkln {
	($($arg:tt)*) => ({
		use ::core::fmt::Write;
		writeln!($crate::init::earlyprintk::EARLY_PRINTK_SERIAL.lock().coms[0].as_mut().unwrap(), $($arg)*)
	});
}
