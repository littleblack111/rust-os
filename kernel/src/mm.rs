use crate::mm::addr::PhysicalMemoryAddress;

pub mod addr;

pub fn init(kernel_virtual_base: PhysicalMemoryAddress) {
    addr::init(kernel_virtual_base);
}
