use spin::{lazylock::LazyLock, mutex::Mutex};

use crate::io::output::serial::uart_16550::Uart16550;

// TODO: use linkme instead
pub static EARLY_PRINTK_SERIAL: LazyLock<Option<Mutex<Uart16550>>> =
    LazyLock::new(|| match unsafe { Uart16550::init([const { None }; 4]) } {
        Ok(o) => Some(Mutex::new(o)),
        Err(_) => None,
    });

#[macro_export]
// TODO: configurable via option or arguments
// or use a system similar to the drivers(`linkme`)
macro_rules! printk {
	($($arg:tt)*) => ({
		use ::core::fmt::Write;
		match &*$crate::init::earlyprintk::EARLY_PRINTK_SERIAL {
			// TODO: elim the unwrap
			Some(m) => write!(m.lock().coms[0].as_mut().unwrap(), $($arg)*),
			// TODO: if needed find a way to return an error here, but for now just ignore it
			None => Ok(()),
		}
	});
}

#[macro_export]
macro_rules! printkln {
	($($arg:tt)*) => ({
		use ::core::fmt::Write;
		match &*$crate::init::earlyprintk::EARLY_PRINTK_SERIAL {
			Some(m) => writeln!(m.lock().coms[0].as_mut().unwrap(), $($arg)*),
			None => Ok(()),
		}
	});
}
