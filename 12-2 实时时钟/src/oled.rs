use embedded_graphics::{
    mono_font::{ascii, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use stm32f1xx_hal::{
    afio::Parts as A_Parts,
    flash::Parts as F_Parts,
    gpio::{gpiob::Parts as G_Parts, Alternate, OpenDrain, Pin as G_Pin},
    i2c::{BlockingI2c, DutyCycle, Mode},
    pac::I2C1,
    prelude::*,
    rcc::CFGR,
};

type I2cType = BlockingI2c<
    I2C1,
    (
        G_Pin<'B', 8, Alternate<OpenDrain>>,
        G_Pin<'B', 9, Alternate<OpenDrain>>,
    ),
>;

pub fn oled_init(
    mut gpiob: G_Parts,
    mut afio: A_Parts,
    mut flash: F_Parts,
    cfgr: CFGR,
    i2c: I2C1,
) -> I2cType {
    let clocks = cfgr.freeze(&mut flash.acr);

    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let oled = BlockingI2c::i2c1(
        i2c,
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

    oled
}

pub fn oled_display(oled: I2cType, text: &str) {
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
