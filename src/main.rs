//! Almond Runtime
#![no_std]
#![no_main]

extern crate alloc;

use core::alloc::Layout;

use alloc::boxed::Box;
use almond_os::{err, log, print, sys::{self, mem::{frame_allocator, malloc, mapper}, terminal::Spinner, timer::{TICKS_PER_SECOND, sleep_ticks, uptime}, vga}};
use bootloader::{BootInfo, entry_point};
use x86_64::{PhysAddr, VirtAddr, instructions::hlt, structures::paging::{Page, Translate}};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    almond_os::boot(boot_info);
    log!("Hello, World!\n\r");
    log!("Bus 0 Present: {}.\n", sys::storage::ata::is_present(0));
    log!("Bus 1 Present: {}.\n", sys::storage::ata::is_present(1));
    let mut spinner = Spinner::new();
    for i in 0.. {
        log!("Uptime: {:0.9}\r", uptime());
        spinner.update();
        sleep_ticks(16);
    }
    almond_os::halt();
}