use core::panic::PanicInfo;

use crate::{pm::hcf, printlnk};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    _ = printlnk!("{info}");
    hcf()
}
