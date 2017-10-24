#![feature(lang_items)]
#![feature(unique)]
#![feature(const_fn)]
#![feature(abi_x86_interrupt)]
#![feature(alloc)]
#![feature(const_unique_new)]
#![feature(asm)]
#![no_std]
#![feature(core_intrinsics)]
#![feature(naked_functions)]
extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate x86;
#[macro_use]
extern crate lazy_static;

extern crate hole_list_allocator as allocator;
#[macro_use]
extern crate alloc;

extern crate bit_field;
extern crate cpuio;
#[macro_use]
mod vga_buffer;
mod memory;
use memory::FrameAllocator;

mod context;
mod time;
#[macro_use]
extern crate once;

extern crate pic8259_simple;
#[macro_use]
mod interrupts;


mod device;

#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    vga_buffer::clear_screen();
    
    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
    
    enable_nxe_bit();
    enable_write_protect_bit();
    
    let mut memory_controller = memory::init(boot_info);
    //frame_allocator.allocate_frame(); 
    //println!("No crash :D");
    //memory::test_paging(&mut frame_allocator);
	
    // initialize our IDT
    //interrupts::init(&mut memory_controller);
    
    unsafe {
        interrupts::idt::init();
        device::pic::init_pic();
        device::pit::init();
        use x86::irq::enable;
        enable();
    }

    println!("It did not crash!");
    
    //use alloc::boxed::Box;
    //let mut heap_test = Box::new(42);
    //*heap_test -= 15;
    //let heap_test2 = Box::new("hello");
    
    //println!("{:x}, {:x}", tstA as *const () as usize, tstB as *const () as usize);     
    //use context::Process;
    //let mut p1 = Process::new(tstA as *const () as usize);
    //let mut idle = Process::new_idle();
    
    //unsafe {idle.regs.switch_to(&mut p1.regs);}
    



    loop {}
}


fn tstA() {
    loop {
        println!("A");
    }
}

fn tstB() {
    loop {

    }
}

fn enable_nxe_bit() {
    use x86::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86::controlregs::{cr0, cr0_write};
    // WRITE_PROTECT
    unsafe { cr0_write(cr0() | 1 << 16) };
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"]
#[no_mangle] 
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop{}
}
