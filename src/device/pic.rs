use device::io::Io;

pub static mut MASTER: Pic = Pic::new(0x20);
pub static mut SLAVE: Pic = Pic::new(0xA0);

pub unsafe fn init_pic() {
    // Init
    MASTER.cmd.write(0x11);
    SLAVE.cmd.write(0x11);

    // offsets
    MASTER.data.write(0x20);
    SLAVE.data.write(0x28);

    // cascade
    MASTER.data.write(4);
    SLAVE.data.write(2);

    // interrupt mode 8086/88
    MASTER.data.write(1);
    SLAVE.data.write(1);

    // unmask interrupts
    MASTER.data.write(0);
    SLAVE.data.write(0);

    // ack
    MASTER.ack();
    SLAVE.ack();
}


pub struct Pic {
    cmd: Io,
    data: Io,
}

impl Pic {
    pub const fn new(port: u16) -> Pic {
        Pic {
            cmd: Io::new(port),
            data: Io::new(port + 1),
        }
    }

    pub fn ack(&mut self) {
        self.cmd.write(0x20);
    }

    pub fn set_mask(&mut self, irq: u8) {
        assert!(irq < 8);

        let mut mask = self.data.read();
        mask |= 1 << irq;
        self.data.write(mask);
    }

    pub fn clear_mask(&mut self, irq: u8) {
        assert!(irq < 8);

        let mut mask = self.data.read();
        mask &= !(1 << irq);
        self.data.write(mask);
    }
}
