# Blink LED on IO3 (ESP32C3) via IDF's LL

And also drives a 132x32 SSD1306 display on IO5 (SDA) / IO6 (SCL)

```bash
cargo espflash /dev/ttyUSB0
```

or build and use `probe-rs-cli run --chip esp32c3 target/riscv32imc-unknown-none-elf/debug/esp32c3` afterward (needs probe-rs cli from branch `run` with _master_ merged in)
