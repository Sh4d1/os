use device::io::Io;
use device::pic;
pub static mut CHANNEL0: Io = Io::new(0x40);
pub static mut COMMAND: Io = Io::new(0x43);


static SELECT_CHANNEL0: u8 = 0;
static LOHI: u8 = 0x30;

static CHANNEL0_DIVISOR: u16 = 2685;

static CLOCKFREQ: u16 = 50;
static QUARTZ: u16 = 0x1234DD;
static SET_FREQ: u8 = 0x34;

pub unsafe fn init() {
    COMMAND.write(SELECT_CHANNEL0 | LOHI | 5);
    //COMMAND.write(SET_FREQ);
    CHANNEL0.write((CHANNEL0_DIVISOR & 0xFF) as u8);
    CHANNEL0.write((CHANNEL0_DIVISOR >> 8) as u8);
    //CHANNEL0.write(((QUARTZ/CLOCKFREQ) & 0xFF) as u8);
    //CHANNEL0.write(((QUARTZ/CLOCKFREQ) >> 8) as u8);
    pic::MASTER.clear_mask(0);

    println!("PIT initialized");
}
