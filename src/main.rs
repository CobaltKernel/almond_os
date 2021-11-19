//! Almond Runtime
#![no_std]
#![no_main]


use almond_os::{print, sys::vga};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    almond_os::boot();
    print!("Hello, World!\n\r");
    almond_os::halt();
}