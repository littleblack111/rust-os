pub static IDT: LazyLock<InterruptDescriptorTable> = LazyLock::new(
    || {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt
    },
);

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{stack_frame:#?}\nERROR CODE:{error_code}");
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    _ = printlnk!("EXCEPTION: BREAKPOINT\n{stack_frame:#?}");
}

pub fn init() {
    IDT.load();
}
