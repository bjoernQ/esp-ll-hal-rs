@echo off
set CHIP=esp32c3
call :compile
set CHIP=esp32
call :compile
exit /b

:compile
echo Generating for %CHIP%
if %CHIP% equ esp32c3 (
    set TARGET=CONFIG_IDF_TARGET_ESP32C3
    set CC=riscv32-esp-elf-gcc
    set XTENSA=
)

if %CHIP% equ esp32 (
    set TARGET=CONFIG_IDF_TARGET_ESP32
    set CC=xtensa-esp32-elf-gcc
    set XTENSA=-I%IDF_PATH%/components/xtensa/include/ -I%IDF_PATH%/components/xtensa/%CHIP%/include/ -mlongcalls
)

%CC% -isystem^
    %IDF_PATH%/components/soc/%CHIP%/include/ ^
        -I%IDF_PATH%/components/hal/%CHIP%/include/ ^
        -I%IDF_PATH%/components/soc/include/ ^
        -I%IDF_PATH%/components/soc/%CHIP%/ ^
        -I%IDF_PATH%/components/esp_hw_support/include/soc/ ^
        -I%IDF_PATH%/components/hal/include/ ^
        -I%IDF_PATH%/components/hal/platform_port/include/ ^
        -I%IDF_PATH%/components/esp_common/include/ ^
        -I%IDF_PATH%/components/esp_rom/include/ ^
        %XTENSA% ^
        -I. ^
        -D %TARGET% ^
        -O2 -c ^
        idf.c ^
        -o libidfhal%CHIP%.a

copy libidfhal%CHIP%.a ..\esp-ll\libs\libidfhal%CHIP%.a
exit /b