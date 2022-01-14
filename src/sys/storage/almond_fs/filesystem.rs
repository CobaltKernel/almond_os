use alloc::{boxed::Box};

use crate::vfs::FileSystem;

pub const DISK_SIZE: usize = (128 << 20) / BLOCK_SIZE;

pub const MAX_ENTRIES: usize = 65536;
pub const MAX_DATA_SIZE: usize = (64 << 20) / BLOCK_SIZE;

pub const BLOCK_SIZE: usize = 512;
pub const BLOCKS_IN_BITMAP: usize = BLOCK_SIZE * 8;
pub const PARTITION_SIZE: usize = (128 << 20) / BLOCK_SIZE;

pub const SUPERBLOCK_START: usize = 0;
pub const SUPERBLOCK_SIZE: usize = 1;

pub const METADATA_BITMAP_START: usize = SUPERBLOCK_SIZE + SUPERBLOCK_START;
pub const METADATA_BITMAP_SIZE: usize = MAX_ENTRIES / BLOCKS_IN_BITMAP; 
pub const METADATA_BITMAP_END: usize = METADATA_BITMAP_SIZE + METADATA_BITMAP_START;

pub const DATA_BITMAP_START: usize = METADATA_BITMAP_START + METADATA_BITMAP_SIZE;
pub const DATA_BITMAP_SIZE: usize = MAX_DATA_SIZE / BLOCKS_IN_BITMAP; 
pub const DATA_BITMAP_END: usize = DATA_BITMAP_SIZE + DATA_BITMAP_START;

pub const METADATA_START: usize = DATA_BITMAP_END;
pub const METADATA_SIZE: usize = MAX_ENTRIES;

pub const DATA_START: usize = METADATA_START + METADATA_SIZE;
pub const DATA_SIZE: usize = MAX_DATA_SIZE;

#[derive(Debug)]
pub struct AlmondFileSystem {
    _private: (),
}

#[allow(unused)]
impl FileSystem for AlmondFileSystem {
    fn block_count(&self) -> usize {
        todo!();
    }

    fn change_dir(&self, dirname: &str) -> Option<Box<dyn crate::vfs::Dir>> {
        todo!();
    }

    fn create_file(&mut self, filename: &str) -> Option<Box<dyn crate::vfs::FileIO>> {
        todo!()
    }

    fn open_file(&mut self, filename: &str) -> Option<Box<dyn crate::vfs::FileIO>> {
        todo!()
    }

    fn create_dir(&mut self, dirname: &str) -> Option<Box<dyn crate::vfs::Dir>> {
        todo!()
    }

    fn current_dir(&self) -> Option<Box<dyn crate::vfs::Dir>> {
        todo!()
    }

    fn file_exists(&self, filename: &str) -> bool {
        todo!()
    }

    fn dir_exists(&self, dirname: &str) -> bool {

        todo!()
    }
}

