#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _;

use cortex_m_rt::entry;

use nrf51_hal as hal;


use rtt_target::{rtt_init_print, rprintln};


#[allow(clippy::empty_loop)]
#[entry]
fn main() -> ! {

    rtt_init_print!();
    rprintln!("Hello!");

    
    loop {

        rprintln!("Hi!");
    }
}
