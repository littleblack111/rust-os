use crate::init::cpu::int::idt::IDT;

pub mod idt;

pub fn init() {
    IDT.load();
}
