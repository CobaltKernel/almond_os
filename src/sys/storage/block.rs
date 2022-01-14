use crate::{vfs::BlockDeviceIO, KResult, slog};

use super::mfs::bitmap_block::BitmapBlock;

pub const BLOCK_SIZE: usize = 512;


#[derive(Debug, Clone, Copy)]
pub struct Block {
    addr: u32,
    buf: [u8; BLOCK_SIZE]
}

impl Block {

    pub fn allocate_mfs() -> Option<Self> {
        if let Some(addr) = BitmapBlock::next_free_addr() {
            BitmapBlock::alloc(addr);
            Some(Self::empty(addr))
        } else {
            None
        }
    }

    pub fn empty(addr: u32) -> Self {
        Self {
            addr,
            buf: [0; BLOCK_SIZE]
        }
    }

    pub fn read(addr: u32) -> KResult<Self> {
        if let Some(dev) = &*crate::vfs::device() {
            //slog!("Reading Block 0x{:08x}\n", addr);
            Ok(Self {
                addr,
                buf: dev.read(addr as usize)?
            })
        } else {
            Err("Device Not Mounted.")
        }
    }

    pub fn write(&self) -> KResult<()> {
        if let Some(dev) = &mut *crate::vfs::device() {
            //slog!("Writing Block 0x{:08x}\n", self.addr);
            dev.write(self.addr as usize, &self.buf)
        } else {
            Err("Device Not Mounted")
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.buf[..]
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.buf[..]
    }

    pub fn addr(&self) -> u32 {
        self.addr
    }
}
