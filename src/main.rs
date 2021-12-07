//! Almond Runtime
#![no_std]
#![no_main]

extern crate alloc;

use core::alloc::Layout;

use alloc::boxed::Box;
use almond_os::{err, log, print, sys::{self, mem::{frame_allocator, malloc, mapper}, terminal::Spinner, timer::{TICKS_PER_SECOND, sleep_ticks, uptime}, vga, storage::nut_fs::{self, metablock::{MetaData, FileType}, partition::KERNEL_SIZE}}, slog};
use bootloader::{BootInfo, entry_point};
use x86_64::{PhysAddr, VirtAddr, instructions::hlt, structures::paging::{Page, Translate}};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    almond_os::boot(boot_info);
    log!("Installing Kernel...\n");


    unsafe {
        nut_fs::format(1);
        nut_fs::install()
    };


    

    

    almond_os::halt();
}

