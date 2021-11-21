#!/bin/bash

function compile() {
    CHIP=$1
    echo "Generating for $CHIP"

    if [ "$CHIP" == "esp32c3" ]; then
        TARGET=CONFIG_IDF_TARGET_ESP32C3
        CC=riscv32-esp-elf-gcc
        XTENSA=
    fi

    if [ "$CHIP" == "esp32" ]; then
        TARGET=CONFIG_IDF_TARGET_ESP32
        CC=xtensa-esp32-elf-gcc
        XTENSA="-I$IDF_PATH/components/xtensa/include/ -I$IDF_PATH/components/xtensa/$CHIP/include/  -mlongcalls"
    fi

    $CC -isystem \
        $IDF_PATH/components/soc/$CHIP/include/ \
            -I$IDF_PATH/components/hal/$CHIP/include/ \
            -I$IDF_PATH/components/soc/include/ \
            -I$IDF_PATH/components/soc/$CHIP/ \
            -I$IDF_PATH/components/esp_hw_support/include/soc/ \
            -I$IDF_PATH/components/hal/include/ \
            -I$IDF_PATH/components/hal/platform_port/include/ \
            -I$IDF_PATH/components/esp_common/include/ \
            -I$IDF_PATH/components/esp_rom/include/ \
            $XTENSA \
            -I. \
            -D $TARGET \
            -O2 -c \
            idf.c \
            -o libidfhal$CHIP.a

    cp libidfhal$CHIP.a ../esp-ll/libs/libidfhal$CHIP.a
}

compile esp32c3
compile esp32
