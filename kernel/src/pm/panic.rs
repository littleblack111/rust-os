use core::panic::PanicInfo;

use crate::{pm::hcf, printlnk};

#[allow(dead_code)]
#[cfg_attr(
    target_os = "none",
    panic_handler
)]
fn panic(info: &PanicInfo) -> ! {
    _ = printlnk!("{info}");
    hcf()
}
