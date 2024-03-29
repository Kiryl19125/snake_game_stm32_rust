#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::prelude::*;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    
    rprintln!("Hello");
    loop {}
}
