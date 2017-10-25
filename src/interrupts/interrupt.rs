use device::pic;
use device::io;
use time;

unsafe fn trigger(irq: u8) {
    if irq < 16 {
        if irq >= 8 {
            pic::SLAVE.set_mask(irq - 8);
            pic::MASTER.ack();
            pic::SLAVE.ack();
        } else {
            //pic::MASTER.set_mask(irq);
            pic::MASTER.ack();
            println!("Heeeey");

            io::Io::new(0x60).read();
        }
    }
}


interrupt!(pit, {
    const PIT_RATE: u64 = 2250286;
    
    {
        let mut offset = time::OFFSET.lock();
        let sum = offset.1 + PIT_RATE;
        offset.1 = sum % 1000000000;
        offset.0 += sum / 1000000000;

        if offset.1 <= 2250286 {
            println!("{}", offset.0);
        }

    }

    //println!("{:?}", time::monotonic());

    pic::MASTER.ack();
});

interrupt!(keyboard, {
    trigger(1);
});
