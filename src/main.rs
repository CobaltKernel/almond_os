//! Almond Runtime
#![no_std]
#![no_main]


use almond_os::{print, sys::{self, vga}};
use bootloader::{BootInfo, entry_point};
use x86_64::{VirtAddr, instructions::hlt};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    almond_os::boot();
    print!("Hello, World!\n\r");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { sys::mem::l4_page_table_at(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            print!("L4 Entry {}: {:?}\n", i, entry);
        }
    }
    almond_os::halt();
}