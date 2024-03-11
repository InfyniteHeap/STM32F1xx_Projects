// #![deny(unsafe_code)]
#![no_std]
#![no_main]

mod oled;

use cortex_m_rt::entry;
use oled::{oled_display, oled_init};
use panic_halt as _;
use stm32f1xx_hal::{pac, prelude::*, rtc::Rtc};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let dp_base = unsafe {
        core::ptr::read(core::ptr::addr_of!(dp) as *mut pac::Peripherals)
    };

    let mut pwr = dp.PWR;
    let rcc = dp.RCC.constrain();
    let bkp = dp.BKP;

    let rcc_base = unsafe {
        core::ptr::read(core::ptr::addr_of!(rcc) as *mut stm32f1xx_hal::rcc::Rcc)
    };

    let mut rtc = Rtc::new(dp.RTC, &mut rcc.bkp.constrain(bkp, &mut pwr));

    rtc.set_time(1710080293);

    let mut buf = itoa::Buffer::new();
    loop {
        let dp_ptr = unsafe {
            core::ptr::read(core::ptr::addr_of!(dp_base) as *mut pac::Peripherals)
        };
        let rcc = unsafe {
            core::ptr::read(core::ptr::addr_of!(rcc_base) as *mut stm32f1xx_hal::rcc::Rcc)
        };
        let gpiob = dp_ptr.GPIOB.split();
        let afio = dp_ptr.AFIO.constrain();
        let flash = dp_ptr.FLASH.constrain();
        let i2c = dp_ptr.I2C1;
        let oled = oled_init(gpiob, afio, flash, rcc.cfgr, i2c);
        oled_display(oled, buf.format(rtc.current_time()));
    }
}
