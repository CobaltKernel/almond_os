//! MorosFS - V1 - Implementation

pub mod super_block;
pub mod bitmap_block;
pub mod dir;
pub mod linked_block;
pub mod read_dir;
pub mod dir_entry;
pub mod api;
pub mod file;

use crate::sys::storage::ata;
use crate::KResult;

use self::bitmap_block::BitmapBlock;
use self::block::Block;
use self::dir::Dir;
use self::file::File;
use self::super_block::SuperBlock;

pub(self) use super::block;
pub(self) use super::block::BLOCK_SIZE;

pub const VERSION: u8 = 1;

pub const KERNEL_SIZE: u32 = 2 << 20; // 2 MiB
pub use bitmap_block::BITMAP_SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Dir = 0,
    File = 1,
    Device = 2,
}


pub fn create_file(name: &str) -> Option<File> {
    File::create(name)
}

pub fn open_file(name: &str) -> Option<File> {
    File::open(name)
}

pub fn root() -> Dir {
    Dir::root()
}

pub fn format() -> KResult<()> {
    unsafe {
        ata::copy(1, 0, 0, (KERNEL_SIZE / 512) as usize);
    }
    

    let sb = SuperBlock::new().unwrap();
    sb.write();
    bitmap_block::free_all();

    root().create_dir("ini");
    root().create_dir("boot");
    root().create_dir("home");
    root().create_dir("lib");
    root().create_dir("bin");




    Ok(())
}