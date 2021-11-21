use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

enum Chip {
    ESP32C3,
    ESP32,
}

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let chip = if env::var("CARGO_FEATURE_ESP32C3").is_ok() {
        Chip::ESP32C3
    } else if env::var("CARGO_FEATURE_ESP32").is_ok() {
        Chip::ESP32
    } else {
        panic!("Need to choose a chip to target. Use a feature of esp-ll");
    };

    let bytes: &[u8] = match chip {
        Chip::ESP32C3 => include_bytes!("./libs/libidfhalesp32c3.a"),
        Chip::ESP32 => include_bytes!("./libs/libidfhalesp32.a"),
    };
    let path = out.join("libidfhal.a");
    File::create(&path).unwrap().write_all(bytes).unwrap();

    let bytes: &[u8] = match chip {
        Chip::ESP32C3 => include_bytes!("./ld/esp32c3.peripherals.ld"),
        Chip::ESP32 => include_bytes!("./ld/esp32.peripherals.ld"),
    };
    let path = out.join("peripherals.ld");
    File::create(&path).unwrap().write_all(bytes).unwrap();

    let bytes: &[u8] = match chip {
        Chip::ESP32C3 => include_bytes!("./ld/esp32c3.rom.api.ld"),
        Chip::ESP32 => include_bytes!("./ld/esp32.rom.api.ld"),
    };
    let path = out.join("rom.api.ld");
    File::create(&path).unwrap().write_all(bytes).unwrap();

    let bytes: &[u8] = match chip {
        Chip::ESP32C3 => include_bytes!("./ld/esp32c3.rom.ld"),
        Chip::ESP32 => include_bytes!("./ld/esp32.rom.ld"),
    };
    let path = out.join("rom.ld");
    File::create(&path).unwrap().write_all(bytes).unwrap();

    println!("cargo:rustc-link-search={}", &out.display());
    println!("cargo:rustc-link-lib=idfhal");

    println!("cargo:rerun-if-changed=./libs");
}
