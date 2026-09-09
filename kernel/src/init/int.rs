use spin::LazyLock;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::printlnk;

pub mod idt;
