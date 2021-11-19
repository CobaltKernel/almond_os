//! Almond Runtime
#![no_std]
#![no_main]


use almond_os::{print, sys::{self, vga}};
use x86_64::instructions::hlt;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    almond_os::boot();
    print!("Hello, World!\n\r");

    loop {
        print!("Ticks: {:04}\r", sys::timer::ticks());
        hlt();
    }

    almond_os::halt();
}