#![no_std]
#![no_main]

// extern crate alloc;

use bootloader_api::{BootInfo, entry_point};
use kernel::{pm::hcf, printlnk};

// const CONFIG: BootloaderConfig = BootloaderConfig::new_default();

fn kmain(_boot_info: &'static mut BootInfo) -> ! {
    _ = printlnk!("test");
    hcf()
}

entry_point!(kmain);
