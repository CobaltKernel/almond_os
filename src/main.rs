//! Almond Runtime
#![no_std]
#![no_main]

extern crate alloc;

use core::alloc::Layout;

use alloc::boxed::Box;
use almond_os::{
    err, log, print, slog,
    sys::{
        self, input,
        mem::{frame_allocator, malloc, mapper, ringbuffer::RingBuffer},
        storage::nut_fs::{
            self,
            metablock::{FileType, MetaData},
            partition::KERNEL_SIZE,
        },
        terminal::Spinner,
        timer::{sleep_ticks, uptime, TICKS_PER_SECOND},
        vga,
    }, shell,
};
use bootloader::{entry_point, BootInfo};
use x86_64::{
    instructions::hlt,
    structures::paging::{Page, Translate},
    PhysAddr, VirtAddr,
};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    almond_os::boot(boot_info);
    shell::main();
    almond_os::halt();
}
