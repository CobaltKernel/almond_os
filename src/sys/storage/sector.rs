//! Abstracts ATA Sectors / Blocks
use core::ops::{Index, IndexMut};

use super::ata::SectorIndex;

/// A Single ATA Sector
#[derive(Debug)]
pub struct Sector {
    addr: SectorIndex,
    data: [u8; 512],
}

impl Sector {
    /// Construct A [Sector] From A [u8] Slice And An Address ([u32]).
    pub fn from(addr: SectorIndex, data: &[u8]) -> Sector {
        let mut buffer = [0; 512];
        for i in 0..data.len() {
            buffer[i] = data[i];
        }
        Sector { addr, data: buffer }
    }

    /// Get A Mutable Reference To The Sector Data
    pub fn data_mut(&mut self) -> &mut [u8] {
        return &mut self.data;
    }

    /// Get A Immutable Reference To The Sector Data
    pub fn data_ref(&self) -> &[u8] {
        return &self.data;
    }

    /// Get The Logical Address Of The Sector.
    pub fn addr(&self) -> SectorIndex {
        self.addr
    }
}

impl Default for Sector {
    fn default() -> Self {
        Self {
            addr: 0,
            data: [0; 512],
        }
    }
}

impl Index<usize> for Sector {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.data[index];
    }
}

impl IndexMut<usize> for Sector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.data[index];
    }
}
