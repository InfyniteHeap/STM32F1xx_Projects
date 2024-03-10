#![deny(unsafe_code)]
#![no_std]
#![no_main]

mod oled;

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use oled::oled_display;
use panic_halt as _;

#[entry]
fn main() -> ! {
    oled_display("1234567890");

    loop {
        nop()
    }
}
