#![no_std]
#![no_main]
#![feature(used_with_arg)]

// extern crate alloc;

use bootloader_api::{BootInfo, BootloaderConfig, entry_point};
use core::fmt::Write;
use kernel::{io::output::serial::uart_16550::Uart16550, pm::hcf, printk};

const CONFIG: BootloaderConfig = BootloaderConfig::new_default();

fn kmain(boot_info: &'static mut BootInfo) -> ! {
    // let mut a = unsafe { Uart16550::init(Default::default()) }.unwrap().com1;
    // write!(a, "test").unwrap();
    printk!("test").unwrap();
    printk!("test").unwrap();
    printk!("test").unwrap();
    printk!("test").unwrap();
    printk!("test").unwrap();
    printk!("test").unwrap();
    printk!("test").unwrap();
    printk!("test").unwrap();
    printk!("test").unwrap();
    hcf()
}

entry_point!(kmain);
