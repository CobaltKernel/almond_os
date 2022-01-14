//! <https://github.com/vinc/moros/blob/trunk/doc/filesystem.md#Superblock>

use crate::{vfs::{device, BlockDeviceIO}, KResult, sys::storage::ata, serr, slog};

use super::{KERNEL_SIZE};
use super::block::*;


const SUPERBLOCK_ADDR: u32 = (KERNEL_SIZE / super::BLOCK_SIZE as u32);
const SIGNATURE: &[u8; 8] = b"MOROS FS";
#[derive(Debug)]
pub struct SuperBlock {
    signature: &'static [u8; 8],
    version: u8,
    block_size: u32,
    pub block_count: u32,
    pub alloc_count: u32,
}

impl SuperBlock {

    pub fn check_dev() -> bool {
        if let Some(dev) = &*device() {
            match dev.read(SUPERBLOCK_ADDR as usize) {
                Ok(buf) => {&buf[0..8] == SIGNATURE},
                Err(_) => {serr!("Failed To Access Device\n"); false}
            }
        } else {
            false
        }
    }

    pub fn new() -> Option<Self> {
        if let Some(dev) = &mut *device() {
            Some(Self {
                alloc_count: 0,
                block_count: dev.block_count() as u32,
                block_size: dev.block_size() as u32,
                signature: SIGNATURE,
                version: super::VERSION,
            })
        } else {
            None
        }
    }

     // NOTE: FS must be mounted
     pub fn read() -> KResult<Self> {
        let block = Block::read(SUPERBLOCK_ADDR)?;
        let data = block.data();
        debug_assert_eq!(&data[0..8], SIGNATURE);
        Ok(
            Self {
                signature: SIGNATURE,
                version: data[8],
                block_size: 2 << (8 + data[9] as u32),
                block_count: u32::from_be_bytes(data[10..14].try_into().unwrap()),
                alloc_count: u32::from_be_bytes(data[14..18].try_into().unwrap()),
            }
        )   
    }

    pub fn write(&self) {
        let mut block = Block::empty(SUPERBLOCK_ADDR);
        let data = block.data_mut();

        data[0..8].clone_from_slice(self.signature);
        data[8] = self.version;

        let size = self.block_size;
        debug_assert!(size >= 512);
        debug_assert!(size.is_power_of_two());
        data[9] = (size.trailing_zeros() as u8) - 9; // 2 ^ (9 + n)
        data[10..14].clone_from_slice(&self.block_count.to_be_bytes());
        data[14..18].clone_from_slice(&self.alloc_count.to_be_bytes());

        block.write().expect("Block Write Failed");
    }

    pub fn block_size(&self) -> u32 {
        self.block_size
    }

    pub fn block_count(&self) -> u32 {
        self.block_count
    }

    pub fn bitmap_area(&self) -> u32 {
        SUPERBLOCK_ADDR + 2
    }

    pub fn data_area(&self) -> u32 {
        let bs = super::BITMAP_SIZE as u32;
        let total = self.block_count;
        let offset = self.bitmap_area();
        let rest = (total - offset) * bs / (bs + 1);
        self.bitmap_area() + rest / bs
    }
}

pub fn inc_alloc_count() {
    let mut sb = SuperBlock::read().expect("Failed To Read SuperBlock");
    sb.alloc_count += 1;
    sb.write();
}

pub fn dec_alloc_count() {
    let mut sb = SuperBlock::read().expect("Failed To Read SuperBlock");
    sb.alloc_count -= 1;
    sb.write();
}