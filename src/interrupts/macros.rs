#[allow(dead_code)]
//#[repr(packed)]
#[derive(Debug)]
#[repr(C)]
pub struct ScratchRegisters {
    pub r11: usize,
    pub r10: usize,
    pub r9: usize,
    pub r8: usize,
    pub rsi: usize,
    pub rdi: usize,
    pub rdx: usize,
    pub rcx: usize,
    pub rax: usize,
}

impl ScratchRegisters {
    pub fn dump(&self) {
        println!("ScratchRegisters:");
        println!("  RAX:   0x{:x}", self.rax);
        println!("  RCX:   0x{:x}", self.rcx);
        println!("  RDX:   0x{:x}", self.rdx);
        println!("  RDI:   0x{:x}", self.rdi);
        println!("  RSI:   0x{:x}", self.rsi);
        println!("  R8:    0x{:x}", self.r8);
        println!("  R9:    0x{:x}", self.r9);
        println!("  R10:   0x{:x}", self.r10);
        println!("  R11:   0x{:x}", self.r11);
    }
}

macro_rules! scratch_push {
    () => (asm!(
            "push rax
            push rcx
            push rdx
            push rdi
            push rsi
            push r8
            push r9
            push r10
            push r11"
            : : : : "intel", "volatile"
        ));
}

macro_rules! scratch_pop {
    () => (asm!(
            "pop r11
            pop r10
            pop r9
            pop r8
            pop rsi
            pop rdi
            pop rdx
            pop rcx
            pop rax"
            : : : : "intel", "volatile"
        ));
}

macro_rules! fs_push {
    () => (asm!(
            "push fs
            mov rax, 0x18
            mov fs, ax"
            : : : : "intel", "volatile"
        ));
}

macro_rules! fs_pop {
    () => (asm!(
            "pop fs"
            : : : : "intel", "volatile"
        ));
}


#[allow(dead_code)]
#[derive(Debug)]
//#[repr(packed)]
#[repr(C)]
pub struct IretRegisters {
    pub rip: usize,
    pub cs: usize,
    pub rflags: usize,
}

impl IretRegisters {
    pub fn dump(&self) {
        println!("IretRegisters:");
        println!("  RFLAG: 0x{:x}", self.rflags);
        println!("  CS:    0x{:x}", self.cs);
        println!("  RIP:   0x{:x}", self.rip);
    }
}


#[derive(Debug)]
#[allow(dead_code)]
//#[repr(packed)]
#[repr(C)]
pub struct InterruptStack {
    //pub fs: usize,
    pub scratch: ScratchRegisters,
    pub iret: IretRegisters,
    pub stack_pointer: usize,
    pub stack_segment: usize,
}

impl InterruptStack {
    pub fn dump(&self) {
        self.scratch.dump();
        self.iret.dump();
        println!("Stack pointer RSP: 0x{:x}", self.stack_pointer);
        println!("Stack segment SS: 0x{:x}", self.stack_segment);
    }
}




macro_rules! iret {
    () => (asm!(
            "iretq"
            : : : : "intel", "volatile"
        ));
}

#[macro_export]
macro_rules! interrupt_stack {
    ($name: ident, $stack: ident, $func: block) => {
        #[naked]
        pub unsafe extern fn $name() {
            #[inline(never)]
            unsafe fn inner($stack: &mut $crate::interrupts::macros::InterruptStack) {
                println!("EXCEPTION!!!");
                $func
            }

            scratch_push!();
            //fs_push!();

			let rsp: usize;
            asm!("" : "={rsp}"(rsp) : : : "intel", "volatile");

            inner(&mut *(rsp as *mut $crate::interrupts::macros::InterruptStack));
            
            //fs_pop!();
            scratch_pop!();
            iret!();
        }
    };
}


#[derive(Debug)]
#[allow(dead_code)]
#[repr(packed)]
pub struct InterruptErrorStack {
    //pub fs: usize,
    pub scratch: ScratchRegisters,
    pub code: usize,
    pub iret: IretRegisters,
    pub stack_pointer: usize,
    pub stack_segment: usize,
}

impl InterruptErrorStack {
    pub fn dump(&self) {
        println!("Error code: {}", self.code);
        self.scratch.dump();
        self.iret.dump();
        println!("Stack pointer RSP: 0x{:x}", self.stack_pointer);
        println!("Stack segment SS: 0x{:x}", self.stack_segment);
    }

    pub fn get_error_code(&self) -> usize {
        self.code
    }
}



#[macro_export]
macro_rules! interrupt_error {
    ($name: ident, $stack: ident, $func: block) => {
        #[naked]
        pub unsafe extern fn $name() {
            #[inline(never)]
            unsafe fn inner($stack: &$crate::interrupts::macros::InterruptErrorStack) {
                println!("EXCEPTION!!!");
                $func
            }

            scratch_push!();
            //fs_push!();

			let rsp: usize;
            asm!("" : "={rsp}"(rsp) : : : "intel", "volatile");
            
            inner(&*(rsp as *mut $crate::interrupts::macros::InterruptErrorStack));

            //fs_pop!();
            scratch_pop!();
            asm!("add rsp, 8" : : : : "intel", "volatile");
            iret!();
        }
    };
}

#[macro_export]
macro_rules! interrupt {
    ($name:ident, $func:block) => {
        #[naked]
        pub unsafe extern fn $name () {
            #[inline(never)]
            unsafe fn inner() {
                $func
            }

            scratch_push!();

            inner();

            scratch_pop!();
            iret!();
        }
    };
}
