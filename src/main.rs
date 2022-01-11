//! Almond Runtime
#![no_std]
#![no_main]

extern crate alloc;
use bootloader::{entry_point, BootInfo};


entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    almond_os::boot(boot_info);
    almond_os::shell::main();
    almond_os::halt();
}
