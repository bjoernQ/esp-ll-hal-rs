#![no_std]

pub const GPIO_MODE_DEF_DISABLE: i32 = 0;
pub const GPIO_MODE_DEF_INPUT: i32 = 1; // bit mask for input
pub const GPIO_MODE_DEF_OUTPUT: i32 = 2; // bit mask for output
pub const GPIO_MODE_DEF_OD: i32 = 3; // bit mask for OD mode

pub const GPIO_MODE_DISABLE: i32 = GPIO_MODE_DEF_DISABLE; // GPIO mode : disable input and output
pub const GPIO_MODE_INPUT: i32 = GPIO_MODE_DEF_INPUT; // GPIO mode : input only
pub const GPIO_MODE_OUTPUT: i32 = GPIO_MODE_DEF_OUTPUT; // GPIO mode : output only mode
pub const GPIO_MODE_OUTPUT_OD: i32 = (GPIO_MODE_DEF_OUTPUT) | (GPIO_MODE_DEF_OD); // GPIO mode : output only with open-drain mode
pub const GPIO_MODE_INPUT_OUTPUT_OD: i32 =
    (GPIO_MODE_DEF_INPUT) | (GPIO_MODE_DEF_OUTPUT) | (GPIO_MODE_DEF_OD); // GPIO mode : output and input with open-drain mode
pub const GPIO_MODE_INPUT_OUTPUT: i32 = (GPIO_MODE_DEF_INPUT) | (GPIO_MODE_DEF_OUTPUT); // GPIO mode : output and input mode

pub const GPIO_PULLUP_ONLY: i32 = 0; // Pad pull up
pub const GPIO_PULLDOWN_ONLY: i32 = 0; // Pad pull down
pub const GPIO_PULLUP_PULLDOWN: i32 = 0; // Pad pull up + pull down
pub const GPIO_FLOATING: i32 = 0; // Pad floating

pub const I2C_SCLK_DEFAULT: i32 = 0; // I2C source clock not selected
pub const I2C_SCLK_APB: i32 = 1; // I2C source clock from APB, 80M
pub const I2C_SCLK_XTAL: i32 = 2; // I2C source clock from XTAL, 40M
pub const I2C_SCLK_RTC: i32 = 3; // I2C source clock from 8M RTC, 8M
pub const I2C_SCLK_REF_TICK: i32 = 4; // I2C source clock from REF_TICK, 1M
pub const I2C_SCLK_MAX: i32 = 5;

pub const I2C_MODE_SLAVE: i32 = 0; // I2C slave mode
pub const I2C_MODE_MASTER: i32 = 1; // I2C master mode
pub const I2C_MODE_MAX: i32 = 2;

extern "C" {
    pub fn gpio_output_enable(gpio_num: i32);
    pub fn gpio_output_disable(gpio_num: i32);
    pub fn gpio_input_disable(gpio_num: i32);
    pub fn gpio_input_enable(gpio_num: i32);
    pub fn gpio_od_disable(gpio_num: i32);
    pub fn gpio_od_enable(gpio_num: i32);
    pub fn gpio_output_set_level(gpio_num: i32, level: u32);
    pub fn gpio_set_direction(gpio_num: i32, mode: i32);
    pub fn gpio_pullup_en(gpio_num: i32);
    pub fn gpio_pullup_dis(gpio_num: i32);
    pub fn gpio_pulldown_en(gpio_num: i32);
    pub fn gpio_pulldown_dis(gpio_num: i32);
    pub fn gpio_set_pull_mode(gpio_num: i32, mode: i32);
    pub fn gpio_iomux_func_sel_gpio(gpio_num: i32);

    pub fn i2c_hw_enable();
    pub fn i2c_hw_disable();
    pub fn i2c0_set_bus_timing(scl_freq: u32, src_clk: i32);
    pub fn i2c0_master_init();
    pub fn i2c0_set_pin(
        sda_io_num: i32,
        scl_io_num: i32,
        sda_pullup_en: bool,
        scl_pullup_en: bool,
        mode: i32,
    );
    pub fn i2c_write_txfifo(ptr: *const u8, len: u8);
    pub fn i2c_trans_start();
    pub fn i2c_write_cmd_0_rstart();
    pub fn i2c_write_cmd_1_write(len: i32);
    pub fn i2c_write_cmd_2_stop();
    pub fn i2c_txfifo_rst();
    pub fn i2c_rxfifo_rst();
    pub fn i2c_update();
    pub fn i2c_is_bus_busy() -> bool;
    pub fn i2c_get_txfifo_cnt() -> i32;
}

#[no_mangle]
extern "C" fn esp_rom_delay_us(us: u32){
    // TODO ... real impl
    for _ in 0..us*100 {
    }
}

/// Naive gpio output implementation
pub struct GpioOutput {
    index: u32
}

impl GpioOutput {
    pub fn new(pin: u32) -> GpioOutput {
        unsafe {
            gpio_iomux_func_sel_gpio(pin as i32);
            gpio_output_enable(pin as i32);
            gpio_pulldown_en(pin as i32);
        }

        GpioOutput { index: pin }
    }
}

impl embedded_hal::digital::v2::OutputPin for GpioOutput {
    type Error = ();

    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe { gpio_output_set_level(self.index as i32, 0); };
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe { gpio_output_set_level(self.index as i32, 1); };
        Ok(())
    }
}

/// Naive i2c output implementation
pub struct I2c {
}

impl I2c {
    pub fn new(sda: u32, scl: u32, sda_pullup: bool, scl_pullup: bool, freq: u32) -> I2c {
        unsafe {
            i2c_hw_enable();
            i2c0_master_init();
            i2c0_set_pin(
                sda as i32, //SDA
                scl as i32, //SCL
                sda_pullup,
                scl_pullup,
                I2C_MODE_MASTER,
            );
            i2c0_set_bus_timing(freq, I2C_SCLK_DEFAULT);
        }

        I2c{}
    }
}

pub enum I2cError {
}

impl embedded_hal::blocking::i2c::Write for I2c {
    type Error = I2cError;

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        unsafe {
            i2c_rxfifo_rst();
            i2c_txfifo_rst();

            let txfifo_cnt = i2c_get_txfifo_cnt();

            i2c_write_cmd_0_rstart();

            i2c_write_txfifo(&[address << 1] as *const _, 1);
            i2c_write_txfifo(bytes.as_ptr(), bytes.len() as u8);
            i2c_write_cmd_1_write((bytes.len() + 1) as i32);
            i2c_write_cmd_2_stop();
            i2c_update();
            i2c_trans_start();

            while txfifo_cnt != i2c_get_txfifo_cnt() {}
        }

        Ok(())
    }
}
