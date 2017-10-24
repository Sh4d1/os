#[naked]
pub extern "C" fn ctx_switch() {
//    unsafe {
  //      asm!("
    //        movl 
}

pub enum Status {
    Runnable,
    Blocked,
    Exited(usize),
}
use alloc::boxed::Box;
pub struct Process {
    pub id: usize,
    pub status: Status,
    pub regs: Registers,
}
impl Process {
    pub fn new(code: usize) -> Process {
        Process {
            id: 1,
            status: Status::Runnable,
            regs: Registers {
                rbx: 0,
                rbp: 0,
                rsp: code,
            }
        }
    }

    pub fn new_idle() -> Process { 
        Process {
            id: 0,
            status: Status::Runnable,
            regs: Registers {
                rbx: 0,
                rbp: 0,
                rsp: 0,
            }
        }
    }
}

pub struct Registers {
    rbx: usize,
    rbp: usize,
    rsp: usize,
}

impl Registers {
    #[inline(never)]
    #[naked]
    pub unsafe fn switch_to(&mut self, next: &mut Registers) {

        asm!("mov $0, rbx" : "=r"(self.rbx) : : "memory" : "intel", "volatile");
        asm!("mov rbx, $0" : : "r"(next.rbx) : "memory" : "intel", "volatile");
   

        asm!("mov $0, rsp" : "=r"(self.rsp) : : "memory" : "intel", "volatile");
    println!("hey");        
asm!("mov rsp, $0" : : "r"(next.rsp) : "memory" : "intel", "volatile");


        asm!("mov $0, rbp" : "=r"(self.rbp) : : "memory" : "intel", "volatile");
        asm!("mov rbp, $0" : : "r"(next.rbp) : "memory" : "intel", "volatile");
        
    }
}
