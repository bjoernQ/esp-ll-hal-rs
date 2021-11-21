#include <stddef.h>
#include "hal/gpio_hal.h"
#include "hal/i2c_hal.h"
#include "esp_rom_gpio.h"
#include "esp_rom_sys.h"
#include "hal/clk_gate_ll.h"

#define I2C_IO_INIT_LEVEL              (1)

#include "gpio_periph.c"
#include "i2c_periph.c"

void gpio_output_enable(unsigned int gpio_num) {
    gpio_ll_output_enable(GPIO_HAL_GET_HW(GPIO_PORT_0), gpio_num);
}

void gpio_input_disable(gpio_num_t gpio_num)
{
    gpio_ll_input_disable(GPIO_HAL_GET_HW(GPIO_PORT_0), gpio_num);
}

void gpio_input_enable(gpio_num_t gpio_num)
{
    gpio_ll_input_enable(GPIO_HAL_GET_HW(GPIO_PORT_0), gpio_num);
}

void gpio_output_disable(gpio_num_t gpio_num)
{
    gpio_ll_output_disable(GPIO_HAL_GET_HW(GPIO_PORT_0), gpio_num);
}

void gpio_od_disable(gpio_num_t gpio_num)
{
    gpio_ll_od_disable(GPIO_HAL_GET_HW(GPIO_PORT_0), gpio_num);
}

void gpio_od_enable(gpio_num_t gpio_num)
{
    gpio_ll_od_enable(GPIO_HAL_GET_HW(GPIO_PORT_0), gpio_num);
}

void gpio_output_set_level(unsigned int gpio_num, unsigned int level) {
    gpio_ll_set_level(GPIO_HAL_GET_HW(GPIO_PORT_0), gpio_num, level);
}


void gpio_set_direction(gpio_num_t gpio_num, gpio_mode_t mode)
{
    if (mode & GPIO_MODE_DEF_INPUT) {
        gpio_input_enable(gpio_num);
    } else {
        gpio_input_disable(gpio_num);
    }

    if (mode & GPIO_MODE_DEF_OUTPUT) {
        gpio_output_enable(gpio_num);
    } else {
        gpio_output_disable(gpio_num);
    }

    if (mode & GPIO_MODE_DEF_OD) {
        gpio_od_enable(gpio_num);
    } else {
        gpio_od_disable(gpio_num);
    }
}

void gpio_pullup_en(gpio_num_t gpio_num)
{
        gpio_ll_pullup_en(GPIO_HAL_GET_HW(GPIO_PORT_0), gpio_num);
}

void gpio_pullup_dis(gpio_num_t gpio_num)
{
        gpio_ll_pullup_dis(GPIO_HAL_GET_HW(GPIO_PORT_0), gpio_num);
}

void gpio_pulldown_en(gpio_num_t gpio_num)
{
    gpio_ll_pulldown_en(GPIO_HAL_GET_HW(GPIO_PORT_0), gpio_num);
}

void gpio_pulldown_dis(gpio_num_t gpio_num)
{
        gpio_ll_pulldown_dis(GPIO_HAL_GET_HW(GPIO_PORT_0), gpio_num);
}

void gpio_set_pull_mode(gpio_num_t gpio_num, gpio_pull_mode_t pull)
{
    switch (pull) {
        case GPIO_PULLUP_ONLY:
            gpio_pulldown_dis(gpio_num);
            gpio_pullup_en(gpio_num);
            break;

        case GPIO_PULLDOWN_ONLY:
            gpio_pulldown_en(gpio_num);
            gpio_pullup_dis(gpio_num);
            break;

        case GPIO_PULLUP_PULLDOWN:
            gpio_pulldown_en(gpio_num);
            gpio_pullup_en(gpio_num);
            break;

        case GPIO_FLOATING:
            gpio_pulldown_dis(gpio_num);
            gpio_pullup_dis(gpio_num);
            break;

        default:
            break;
    }
}

void gpio_iomux_func_sel_gpio(int pin) {
    gpio_hal_iomux_func_sel(GPIO_PIN_MUX_REG[pin], PIN_FUNC_GPIO);
}

void i2c_hw_disable()
{
    int i2c_num  = 0;
    periph_ll_disable_clk_set_rst(i2c_periph_signal[i2c_num].module);
}

void i2c_hw_enable()
{
    int i2c_num  = 0;
    periph_ll_enable_clk_clear_rst(i2c_periph_signal[i2c_num].module);
}

void i2c_txfifo_rst() {
  i2c_ll_txfifo_rst(I2C_LL_GET_HW(0));
}

void i2c_rxfifo_rst() {
  i2c_ll_rxfifo_rst(I2C_LL_GET_HW(0));
}

void i2c_update() {
  i2c_ll_update(I2C_LL_GET_HW(0));
}

void i2c0_set_bus_timing(int scl_freq, i2c_sclk_t src_clk)
{
    i2c_ll_set_source_clk(I2C_LL_GET_HW(0), src_clk);
    uint32_t sclk = I2C_LL_CLK_SRC_FREQ(src_clk);
    i2c_clk_cal_t clk_cal = {0};
    uint32_t scl_hw_freq = (scl_freq == I2C_CLK_FREQ_MAX) ? (sclk / 20) : (uint32_t)scl_freq; // FREQ_MAX use the highest freq of the chosen clk.
    i2c_ll_cal_bus_clk(sclk, scl_hw_freq, &clk_cal);
    i2c_ll_set_bus_timing(I2C_LL_GET_HW(0), &clk_cal);
}

void i2c0_master_init() {
    i2c_ll_master_init(I2C_LL_GET_HW(0));
    //Use fifo mode
    i2c_ll_set_fifo_mode(I2C_LL_GET_HW(0), true);
    //MSB
    i2c_ll_set_data_mode(I2C_LL_GET_HW(0), I2C_DATA_MODE_MSB_FIRST, I2C_DATA_MODE_MSB_FIRST);
    //Reset fifo
    i2c_ll_txfifo_rst(I2C_LL_GET_HW(0));
    i2c_ll_rxfifo_rst(I2C_LL_GET_HW(0));    
}

void i2c0_set_pin(int sda_io_num, int scl_io_num, bool sda_pullup_en, bool scl_pullup_en, i2c_mode_t mode)
{
    int i2c_num = 0;
    int sda_in_sig, sda_out_sig, scl_in_sig, scl_out_sig;
    sda_out_sig = i2c_periph_signal[i2c_num].sda_out_sig;
    sda_in_sig = i2c_periph_signal[i2c_num].sda_in_sig;
    scl_out_sig = i2c_periph_signal[i2c_num].scl_out_sig;
    scl_in_sig = i2c_periph_signal[i2c_num].scl_in_sig;

    if (sda_io_num >= 0) {
        gpio_output_set_level(sda_io_num, I2C_IO_INIT_LEVEL);
        gpio_hal_iomux_func_sel(GPIO_PIN_MUX_REG[sda_io_num], PIN_FUNC_GPIO);
        gpio_set_direction(sda_io_num, GPIO_MODE_INPUT_OUTPUT_OD);

        if (sda_pullup_en == GPIO_PULLUP_ENABLE) {
            gpio_set_pull_mode(sda_io_num, GPIO_PULLUP_ONLY);
        } else {
            gpio_set_pull_mode(sda_io_num, GPIO_FLOATING);
        }
        esp_rom_gpio_connect_out_signal(sda_io_num, sda_out_sig, 0, 0);
        esp_rom_gpio_connect_in_signal(sda_io_num, sda_in_sig, 0);
    }
    if (scl_io_num >= 0) {
        gpio_output_set_level(scl_io_num, I2C_IO_INIT_LEVEL);
        gpio_hal_iomux_func_sel(GPIO_PIN_MUX_REG[scl_io_num], PIN_FUNC_GPIO);
        gpio_set_direction(scl_io_num, GPIO_MODE_INPUT_OUTPUT_OD);
        esp_rom_gpio_connect_out_signal(scl_io_num, scl_out_sig, 0, 0);
        esp_rom_gpio_connect_in_signal(scl_io_num, scl_in_sig, 0);
        if (scl_pullup_en == GPIO_PULLUP_ENABLE) {
            gpio_set_pull_mode(scl_io_num, GPIO_PULLUP_ONLY);
        } else {
            gpio_set_pull_mode(scl_io_num, GPIO_FLOATING);
        }
    }

}

// i2c write
// see https://www.espressif.com/sites/default/files/documentation/esp32_technical_reference_manual_en.pdf page 286

/**
 * @brief  Write the I2C hardware txFIFO
 *
 * @param  ptr Pointer to data buffer
 * @param  len Amount of data needs to be writen
 *
 * @return None.
 */
void i2c_write_txfifo(uint8_t *ptr, uint8_t len){
    i2c_ll_write_txfifo(I2C_LL_GET_HW(0),ptr,len);
}

/**
 * @brief  Start I2C transfer
 *
 * @return None
 */
void i2c_trans_start()
{
    i2c_ll_trans_start(I2C_LL_GET_HW(0));
}

void i2c_write_cmd_0_rstart()
{
    i2c_hw_cmd_t cmd = { 0 };
    cmd.op_code = I2C_LL_CMD_RESTART;
    i2c_ll_write_cmd_reg(I2C_LL_GET_HW(0), cmd, 0);
}

void i2c_write_cmd_1_write(int len)
{
    i2c_hw_cmd_t cmd = { 0 };
    cmd.op_code = I2C_LL_CMD_WRITE;
    cmd.byte_num = len;
    i2c_ll_write_cmd_reg(I2C_LL_GET_HW(0), cmd, 1);
}

void i2c_write_cmd_2_stop(int len)
{
    i2c_hw_cmd_t cmd = { 0 };
    cmd.op_code = I2C_LL_CMD_STOP;
    i2c_ll_write_cmd_reg(I2C_LL_GET_HW(0), cmd, 2);
}

bool i2c_is_bus_busy()
{
    return i2c_ll_is_bus_busy(I2C_LL_GET_HW(0));
}

int i2c_get_txfifo_cnt()
{
    return i2c_ll_get_txfifo_len(I2C_LL_GET_HW(0));
}
