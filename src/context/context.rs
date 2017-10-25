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
    pub kstack: Option<Box<[u8]>>,
}
impl Process {
    pub fn new(id: usize) -> Process {
        Process {
            id: id,
            status: Status::Runnable,
            regs: Registers {
                rbx: 0,
                rbp: 0,
                rsp: 0,
            },
            kstack: None,
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
