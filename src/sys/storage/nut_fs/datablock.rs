//! TODO

use alloc::vec::Vec;

use crate::{
    slog,
    sys::storage::{ata, nut_fs::partition::DISK_SIZE},
    KResult,
};

use super::data_bitmap::DataBitmap;

#[derive(Debug, Clone, Copy)]
/// A Singly Linked List Data Block. 0: This Is The Final Block.
pub struct DataBlock {
    addr: u32,

    next: u32,
    data: [u8; 508],
}

impl DataBlock {
    /// Reads A [Sector] Data Block From Disk
    pub fn read(drive: usize, addr: u32) -> KResult<Self> {
        let buffer = ata::read_block(drive, addr)?;
        let mut data = [0; 508];
        data.copy_from_slice(&buffer[4..]);
        Ok(Self {
            addr,

            data,
            next: u32::from_be_bytes((&buffer[0..4]).try_into().unwrap()),
        })
    }

    /// Write This Data Block To Disk
    pub fn write(&self, drive: usize) -> KResult<()> {
        let mut buffer = [0; 512];
        buffer[0..4].copy_from_slice(&self.next.to_be_bytes());
        buffer[4..].copy_from_slice(&self.data);

        slog!("Writing Block 0x{:04x}\n", self.addr);

        ata::write(drive, self.addr, &buffer)
    }

    /// Immutable Reference To The Underlying Data
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Mutable Reference To The Underlying Data
    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    /// The Next Block Index. Final Is NULL (0x00).
    pub fn next(&self) -> u32 {
        self.next
    }

    /// Update The Next Index
    pub fn set_next(&mut self, value: u32) {
        slog!(
            "[Block 0x{:04x}]: Setting Next To Block 0x{:04x}\n",
            self.addr,
            value
        );
        self.next = value;
    }

    /// This Block Index.
    pub fn addr(&self) -> u32 {
        self.addr
    }

    /// Create An empty Data Block.
    pub fn create(addr: u32) -> Self {
        assert!(addr < DISK_SIZE, "Block Out Of Range.");
        slog!("Creating A New Data Block At 0x{:04x}\n", addr);
        Self {
            addr,
            next: 0,
            data: [0; 508],
        }
    }

    /// Allocate An empty Block
    pub fn allocate(drive: usize) -> KResult<Self> {
        if let Some(block) = DataBitmap::next_free(drive) {
            DataBitmap::allocate(drive, block);
            return Ok(Self::create(block));
        } else {
            return Err("Disk Full.");
        }
    }
}

/// Read A List Of Blocks, Cloning Them Into buffer.
pub fn read_blocks(drive: usize, addr: u32, buffer: &mut Vec<DataBlock>) -> KResult<()> {
    let mut current = DataBlock::read(drive, addr)?;
    let mut addr;
    buffer.push(current.clone());
    while current.next() > 0 {
        addr = current.next();
        buffer.push(current.clone());
        current = DataBlock::read(drive, addr)?;
    }
    Ok(())
}
