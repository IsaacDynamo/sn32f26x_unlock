#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

// Bootloader 0x7800 BLOB
#[link_section = ".data"]
static ROM_7800 : [u8;1536] = *include_bytes!("../ROM_7800.bin");

// Minimal interrupt vector table
#[link_section = ".data"]
static IVT : [u32;2] = [
    0x2000_0800,    // Setup stack at the end of RAM
    0x1FFF_0009     // Jump directly to the bootloader
];

// Declare function located in bootloader ROM
extern "C" { 
    fn rom_addr_bootloader() -> !; 
    fn rom_addr_cs_erase(); 
    fn rom_addr_flash_prog(addr: u32, len: u32, data: *const u8) -> bool; 
}

#[inline(always)]
fn bootloader() -> ! {
    unsafe { rom_addr_bootloader() }
}

#[inline(always)]
fn erase_cs() {
    unsafe { rom_addr_cs_erase() }
}

#[inline(always)]
fn program_flash<T>(addr: u32, data: &[T]) {
    // Convert generic slice to bytes
    let bytes: &[u8] = unsafe { core::slice::from_raw_parts::<u8>(data.as_ptr() as * const u8, data.len() * core::mem::size_of::<T>() ) };

    // Program the data bytes in chunks of 64 bytes.
    let mut addr = addr;
    for page in bytes.chunks(64) {
        unsafe { rom_addr_flash_prog(addr, page.len() as u32, page.as_ptr()) };
        addr += page.len() as u32;
    }
}

#[entry]
fn entry() -> ! {
    // Jump to the unlock() func in RAM
    unlock()
}

#[inline(never)]
#[link_section = ".data"]
fn unlock() -> ! {
    // Erase CS, this will whipe the whole flash, including the 0x7800 part of the bootloader
    erase_cs();

    // Restore the 0x7800 bootloader part 
    program_flash(0x7800, &ROM_7800);

    // Flash minimal IVT
    program_flash(0x0, &IVT);

    // Jump to the bootloader
    bootloader();
}
