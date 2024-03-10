use embedded_graphics::{
    mono_font::{ascii, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use stm32f1xx_hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    pac::Peripherals,
    prelude::*,
};

pub fn oled_display(text: &str) {
    let dp = Peripherals::take().unwrap();

    let mut gpiob = dp.GPIOB.split();
    let mut afio = dp.AFIO.constrain();
    let mut flash = dp.FLASH.constrain();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let oled = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400.kHz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        1000,
        10,
        1000,
        1000,
    );

    let interface = I2CDisplayInterface::new(oled);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&ascii::FONT_7X14)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline(text, Point::zero(), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();
}
