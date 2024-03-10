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

    let mut pwr = dp.PWR;
    let rcc = dp.RCC.constrain();
    let bkp = dp.BKP;

    let mut rtc = Rtc::new(dp.RTC, &mut rcc.bkp.constrain(bkp, &mut pwr));

    rtc.set_time(1710080293);

    let mut buf = itoa::Buffer::new();
    loop {
        let gpiob = dp.GPIOB.split();
        let afio = dp.AFIO.constrain();
        let flash = dp.FLASH.constrain();
        let i2c = dp.I2C1;
        let oled = oled_init(gpiob, afio, flash, rcc.cfgr, i2c);
        oled_display(oled, buf.format(rtc.current_time()));
    }
}
