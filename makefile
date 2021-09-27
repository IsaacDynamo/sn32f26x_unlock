all:
	cargo build --release
	arm-none-eabi-objcopy -O binary target/thumbv6m-none-eabi/release/sn32f26x_unlock sn32f26x_unlock.bin
