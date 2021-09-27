/* Linker script for the SN32F26x */
MEMORY
{
    /* Expect jumploader to be at 0x0 - 0x1FF */ 
    FLASH : ORIGIN = 0x00000200, LENGTH = 30K - 512
    RAM   : ORIGIN = 0x20000000, LENGTH = 2K
}

/* Routines in the SN32F26x bootloader ROM */
rom_addr_bootloader = 0x1FFF0008;
rom_addr_cs_erase   = 0x1FFF01C6;
rom_addr_flash_prog = 0x1FFF01CA;
