#![no_std]
#![no_main]

// extern crate alloc;

use bootloader_api::{BootInfo, entry_point};
use kernel::{init, pm::hcf, printlnk};

// const CONFIG: BootloaderConfig = BootloaderConfig::new_default();

fn kmain(_boot_info: &'static mut BootInfo) -> ! {
    init::init();
    x86_64::instructions::interrupts::int3();
    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    };
    _ = printlnk!("test");
    hcf()
}

entry_point!(kmain);
