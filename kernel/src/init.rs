pub mod acpi;
pub mod earlyprintk;
pub mod int;

pub fn init() {
    int::init();
}
