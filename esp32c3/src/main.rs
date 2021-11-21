#![no_std]
#![no_main]
#![feature(asm)]

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::Point,
    text::{Baseline, Text},
    Drawable,
};
use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;
use ssd1306::{mode::DisplayConfig, rotation::DisplayRotation, size, I2CDisplayInterface, Ssd1306};

use core::fmt::Write;
use riscv_rt::entry;

use esp32c3_lib::{disable_wdts, Uart};

use esp_ll::*;

// make sure we have something in our data section
#[used]
static DATA_SECTION_TEST: &'static str = "TEST DATA";
// make sure we have something in our bss section
#[used]
static mut BSS_SECTION_TEST: [u8; 12] = [0xAA; 12];

#[entry]
fn main() -> ! {
    // disable interrupts, `csrwi        mie,0` throws an exception on the esp32c3
    unsafe {
        let mut _tmp: u32;
        asm!("csrrsi {0}, mstatus, {1}", out(reg) _tmp, const 0x00000008)
    };

    // disable wdt's
    disable_wdts();

    writeln!(Uart, "Observe IO5/6 for I2C!").unwrap();
    writeln!(Uart, "{}", DATA_SECTION_TEST).unwrap();

    const GPIO_NUM: u32 = 3;
    let mut led = GpioOutput::new(GPIO_NUM);

    let i2c = I2c::new(5, 6, false, false, 190_000);

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, size::DisplaySize128x32, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();
    display.clear();
    display.flush().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    let mut state = true;
    loop {
        if state {
            led.set_high().ok();
        } else {
            led.set_low().ok();
        }
        state = !state;

        display.clear();

        Text::with_baseline("Hello ESP32C3", Point::zero(), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        Text::with_baseline(
            if state { "Hello Rust!" } else { "Hola Rust!" },
            Point::new(0, 16),
            text_style,
            Baseline::Top,
        )
        .draw(&mut display)
        .unwrap();

        display.flush().unwrap();

        for _ in 0..50000 {}
    }
}
