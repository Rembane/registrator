all: program

program: tobin
	st-flash --reset write registrator.bin 0x8000000

tobin: build
	arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabi/release/registrator registrator.bin

build:
	xargo build --target=thumbv7em-none-eabi --release
