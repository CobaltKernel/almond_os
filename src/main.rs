//! Almond Runtime
#![no_std]
#![no_main]

extern crate alloc;
use almond_os::{print, pci, slog, sys::storage::mfs::{self, super_block::SuperBlock, api::FileIO, dir::Dir, dir_entry::DirEntry}};
use bootloader::{entry_point, BootInfo};


entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    almond_os::boot(boot_info);
    #[cfg(feature = "shell")]
    {
        almond_os::shell::main();
    }



    

    


    almond_os::halt();
}
