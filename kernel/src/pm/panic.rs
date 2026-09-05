use core::panic::PanicInfo;

use crate::pm::hcf;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    hcf()
}
