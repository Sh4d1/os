interrupt_stack!(divide_by_zero_handler, stack, {
    println!("Divide by zero");
    stack.dump();
    loop {}
});

interrupt_stack!(debug_handler, stack, {
    println!("Debug trap");
    stack.dump();
    loop {}
});

interrupt_stack!(non_maskable_handler, stack, {
    println!("Non-maskable interrupt");
    stack.dump();
    loop {}
});

interrupt_stack!(breakpoint_handler, stack, {
    println!("Breakpoint trap");
});

interrupt_stack!(overflow_handler, stack, {
    println!("Overflow trap");
    stack.dump();
    loop {}
});

interrupt_stack!(bound_range_handler, stack, {
    println!("Bound range exceeded fault");
    stack.dump();
    loop {}
});

interrupt_stack!(invalid_opcode_handler, stack, {
    println!("Invalid opcode fault");
    stack.dump();
    loop {}
});

interrupt_stack!(device_not_available_handler, stack, {
    println!("Device not available fault");
    stack.dump();
    loop {}
});

interrupt_error!(double_fault_handler, stack, {
    println!("Double fault");
    stack.dump();
    loop {}
});

interrupt_error!(invalid_tss_handler, stack, {
    println!("Invalid TSS fault");
    stack.dump();
    loop {}
});

interrupt_error!(segment_not_present_handler, stack, {
    println!("Segment not present fault");
    stack.dump();
    loop {}
});

interrupt_error!(stack_segment_handler, stack, {
    println!("Stack segment fault");
    stack.dump();
    loop {}
});

interrupt_error!(protection_handler, stack, {
    println!("Protection fault");
    stack.dump();
    loop {}
});

bitflags! {
    struct PageFaultErrorCode: usize {
        const PROTECTION_VIOLATION = 1 << 0;
        const CAUSED_BY_WRITE = 1 << 1;
        const USER_MODE = 1 << 2;
        const MALFORMED_TABLE = 1 << 3;
        const INSTRUCTION_FETCH = 1 << 4;
    }
}

use x86::controlregs::cr2;

interrupt_error!(page_fault_handler, stack, {
    println!("Page fault: 0x{:x}", cr2());
	println!("Error code: {:?}", PageFaultErrorCode::from_bits(stack.get_error_code()).unwrap());
    stack.dump();
    loop {}
});

interrupt_error!(fpu_handler, stack, {
    println!("FPU floating point fault");
    stack.dump();
    loop {}
});

interrupt_error!(alignment_check_handler, stack, {
    println!("Alignment check fault");
    stack.dump();
    loop {}
});

interrupt_stack!(machine_check_handler, stack, {
    println!("Machine check fault");
    stack.dump();
    loop {}
});

interrupt_stack!(simd_handler, stack, {
    println!("SIMD floating point fault");
    stack.dump();
    loop {}
});

interrupt_stack!(virtualization_handler, stack, {
    println!("Virtualization fault");
    stack.dump();
    loop {}
});

interrupt_error!(security_handler, stack, {
    println!("Security exception");
    stack.dump();
    loop {}
});


