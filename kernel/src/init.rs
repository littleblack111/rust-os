use crate::init::cpu::int;

pub mod acpi;
pub mod earlyprintk;
pub mod cpu;

pub fn init() {
    int::init();
}
