#![no_std]
#![no_main]

use core::panic::PanicInfo;

use embedded_graphics::{Drawable, mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10}, pixelcolor::BinaryColor, prelude::Point, text::{Baseline, Text}};
use esp32_hal::{clock_control::ClockControlConfig, dprintln, prelude::*, target};

use esp_ll::*;
use ssd1306::{I2CDisplayInterface, Ssd1306, mode::DisplayConfig, rotation::DisplayRotation, size};

const WDT_WKEY_VALUE: u32 = 0x50D83AA1;

#[entry]
fn main() -> ! {
    let dp = target::Peripherals::take().expect("Failed to obtain Peripherals");

    let mut rtccntl = dp.RTCCNTL;
    let mut timg0 = dp.TIMG0;
    let mut timg1 = dp.TIMG1;

    // (https://github.com/espressif/openocd-esp32/blob/97ba3a6bb9eaa898d91df923bbedddfeaaaf28c9/src/target/esp32.c#L431)
    // openocd disables the wdt's on halt
    // we will do it manually on startup
    disable_timg_wdts(&mut timg0, &mut timg1);
    disable_rtc_wdt(&mut rtccntl);

    dprintln!("Let's go!");

    let i2c = I2c::new(21, 22, false, false, 190_000);

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
        state = !state;

        display.clear();

        Text::with_baseline("Hello ESP32", Point::zero(), text_style, Baseline::Top)
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

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // park the other core
    unsafe { ClockControlConfig {}.park_core(esp32_hal::get_other_core()) };

    // print panic message
    dprintln!("\n\n*** Core: {:?} {:?}", esp32_hal::get_core(), info);

    // park this core
    unsafe { ClockControlConfig {}.park_core(esp32_hal::get_core()) };

    dprintln!("\n\n Should not reached because core is parked!!!");

    // this statement will not be reached, but is needed to make this a diverging function
    loop {}
}

fn disable_rtc_wdt(rtccntl: &mut target::RTCCNTL) {
    /* Disables the RTCWDT */
    rtccntl
        .wdtwprotect
        .write(|w| unsafe { w.bits(WDT_WKEY_VALUE) });
    rtccntl.wdtconfig0.modify(|_, w| unsafe {
        w.wdt_stg0()
            .bits(0x0)
            .wdt_stg1()
            .bits(0x0)
            .wdt_stg2()
            .bits(0x0)
            .wdt_stg3()
            .bits(0x0)
            .wdt_flashboot_mod_en()
            .clear_bit()
            .wdt_en()
            .clear_bit()
    });
    rtccntl.wdtwprotect.write(|w| unsafe { w.bits(0x0) });
}

fn disable_timg_wdts(timg0: &mut target::TIMG0, timg1: &mut target::TIMG1) {
    timg0
        .wdtwprotect
        .write(|w| unsafe { w.bits(WDT_WKEY_VALUE) });
    timg1
        .wdtwprotect
        .write(|w| unsafe { w.bits(WDT_WKEY_VALUE) });

    timg0.wdtconfig0.write(|w| unsafe { w.bits(0x0) });
    timg1.wdtconfig0.write(|w| unsafe { w.bits(0x0) });
}
