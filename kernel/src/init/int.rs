use crate::init::int::idt::IDT;

pub mod idt;

pub fn init() {
    IDT.load();
}
