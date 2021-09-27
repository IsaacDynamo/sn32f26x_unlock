# SN32F26x unlock

> ⚠️ Only use this if you know what you are doing!

The sn32f26x_unlock.bin firmware will erase the 'Code Security' register of a SN32F26x chip. This will allow SWD to access flash and RAM, making debuging easier. 

A side effect of erasing CS is that the whole flash is erase. This includes a part of the bootloader. This firmware will restore the erased bootloader, and boots into the bootloader.

This image expects to be run on a chip that has a [jumploader](https://github.com/SonixQMK/sonix-keyboard-bootloader). The firmware can be flashed with the [Sonix flasher](https://github.com/SonixQMK/sonix-flasher).
