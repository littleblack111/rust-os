#![no_std]
#![no_main]
#![feature(used_with_arg)]

// extern crate alloc;

use bootloader_api::{BootInfo, BootloaderConfig, entry_point};
use kernel::pm::hcf;

const CONFIG: BootloaderConfig = BootloaderConfig::new_default();

fn kmain(boot_info: &'static mut BootInfo) -> ! {
    hcf()
}

entry_point!(kmain);
