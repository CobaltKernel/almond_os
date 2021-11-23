//! Almond Runtime
#![no_std]
#![no_main]

extern crate alloc;

use core::alloc::Layout;

use alloc::boxed::Box;
use almond_os::{err, log, print, sys::{self, mem::{frame_allocator, malloc, mapper}, timer::{TICKS_PER_SECOND, sleep_ticks}, vga}};
use bootloader::{BootInfo, entry_point};
use x86_64::{PhysAddr, VirtAddr, instructions::hlt, structures::paging::{Page, Translate}};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    almond_os::boot(boot_info);
    log!("Hello, World!\n\r");
    let indicator = r"|/-\";
    for i in 0.. {
        log!("Doing Something {}     \r", &indicator.chars().nth(i % 4).unwrap());
        sleep_ticks(100);
    }
    almond_os::halt();
}