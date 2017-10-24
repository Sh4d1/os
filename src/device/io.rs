//use core::marker::PhantomData;

#[derive(Copy, Clone)]
pub struct Io {
    port: u16,
    //value: PhantomData<T>,
}

impl Io {
    pub const fn new(port: u16) -> Self {
        Io {
            port: port,
            //valule: PhantomData,
        }
    }

    #[inline(always)]
    pub fn read(&self) -> u8 {
        let value: u8;
        unsafe {
            asm!("in $0, $1" : "={al}"(value) : "{dx}"(self.port) : "memory" : "intel", "volatile");
        }
        value
    }

    #[inline(always)]
    pub fn write(&mut self, value: u8) {
        unsafe {
            asm!("out $1, $0" : : "{al}"(value), "{dx}"(self.port) : "memory" : "intel", "volatile");
        }
    }
}


