use x86::dtables::{self, DescriptorTablePointer};

use core::mem;
use interrupts::exception::*;
use interrupts::interrupt::*;
pub static mut IDTR: DescriptorTablePointer = DescriptorTablePointer {
    limit: 0,
    base: 0,
};

pub static mut IDT: [IdtEntry; 256] = [IdtEntry::new(); 256];

pub unsafe fn init() {
    IDTR.limit = (IDT.len() * mem::size_of::<IdtEntry>() - 1) as u16;
    IDTR.base = IDT.as_ptr() as u64;

    IDT[0].set_func(divide_by_zero_handler);
    IDT[1].set_func(debug_handler);
    IDT[2].set_func(non_maskable_handler);
    IDT[3].set_func(breakpoint_handler);
    IDT[4].set_func(overflow_handler);
    IDT[5].set_func(bound_range_handler);
    IDT[6].set_func(invalid_opcode_handler);
    IDT[7].set_func(device_not_available_handler);
    IDT[8].set_func(double_fault_handler);

    IDT[10].set_func(invalid_tss_handler);
    IDT[11].set_func(segment_not_present_handler);
    IDT[12].set_func(stack_segment_handler);
    IDT[13].set_func(protection_handler);
    IDT[14].set_func(page_fault_handler);

    IDT[16].set_func(fpu_handler);
    IDT[17].set_func(alignment_check_handler);
    IDT[18].set_func(machine_check_handler);
    IDT[19].set_func(simd_handler);
    IDT[20].set_func(virtualization_handler);

    IDT[30].set_func(security_handler);
    
    IDT[32].set_func(pit);
    IDT[33].set_func(keyboard);
    dtables::lidt(&IDTR);
    
}


#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct IdtEntry {
    pointer_low: u16,
    gdt_selector: u16,
    zero: u8,
    options: u8,
    pointer_middle: u16,
    pointer_high: u32,
    reserved: u32,
}


bitflags! {
    pub struct IdtFlags: u8 {
        const PRESENT = 1 << 7;
        const RING_0 = 0 << 5;
        const RING_1 = 1 << 5;
        const RING_2 = 2 << 5;
        const RING_3 = 3 << 5;
        const SS = 1 << 4;
        const INTERRUPT = 0xE;
        const TRAP = 0xF;
    }
}

impl IdtEntry {
    pub const fn new() -> IdtEntry {
        IdtEntry {
            gdt_selector: 0,
            pointer_low: 0,
            pointer_middle: 0,
            pointer_high: 0,
            zero: 0,
            options: 0,
            reserved: 0,
        }
    }


    pub fn set_flags(&mut self, flags: IdtFlags) {
        self.options = flags.bits;
    }

    pub fn set_pointer(&mut self, selector: u16, base: usize) {
        self.gdt_selector = selector;
        self.pointer_low = base as u16;
        self.pointer_middle = (base >> 16) as u16;
        self.pointer_high = (base >> 32) as u32;
    }

    pub fn set_func(&mut self, func: unsafe extern fn()) {
        self.set_flags(IdtFlags::PRESENT | IdtFlags::RING_0 | IdtFlags::INTERRUPT);
        self.set_pointer(8, func as usize);
    }
}





