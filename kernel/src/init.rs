pub mod acpi;
pub mod cpu;
pub mod earlyprintk;

pub fn init() {
    cpu::int::init();
}
