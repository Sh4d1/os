use device::pic;
use time;

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


