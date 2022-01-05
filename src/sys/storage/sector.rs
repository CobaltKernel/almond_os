//! Abstracts ATA Sectors / Blocks
use core::ops::{Index, IndexMut};

use alloc::string::String;

use crate::KResult;

use super::ata::{SectorIndex, self};

/// A Single ATA Sector
#[derive(Debug)]
pub struct Sector {
    addr: SectorIndex,
    drive: usize,
    data: [u8; 512],
}

impl Sector {

    /// Read This Sector From Disk.
    pub fn read(drive: usize, addr: u32) -> KResult<Sector> {
        let data = ata::read_block(drive, addr)?;
        Ok(Sector {
            addr,
            data,
            drive,
        })
    }

    /// Write This Sector Out Onto The Disk.
    pub fn write(&self) -> KResult<()> {
        ata::write(self.drive, self.addr, self.data_ref())
    }

    /// Construct A [Sector] From A [u8] Slice And An Address ([u32]).
    pub fn from(addr: SectorIndex, data: &[u8]) -> Sector {
        let mut buffer = [0; 512];
        for i in 0..data.len() {
            buffer[i] = data[i];
        }
        Sector { addr, data: buffer, drive: 1}
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

    /// Read A ASCIIZ String, Returns The String & The Number Of Bytes Read (Terminator Included)
    pub fn read_string_asciiz(&self, pos: usize) -> (String, usize) {
        let data = self.data;
        let mut output = String::new();
        let mut count = 0;
        for idx in pos..data.len() {
            if data[idx] > 0 {
                output.push(data[idx] as char);
                count += 1;
            } else {
                count += 1;
                break;
            }
        }
        return (output, count);
    }


    /// Write An ASCIIZ String, The Number Of Bytes Written (Terminator Included)
    pub fn write_string_asciiz(&mut self, pos: usize, text: String) -> usize {
        let mut count = 0;
        for (index, byte) in text.as_bytes().iter().enumerate() {
            self.data[index + pos] = *byte;
            count += 1;
        }
        count += 1;

        return count;
    }  
}

impl Default for Sector {
    fn default() -> Self {
        Self {
            addr: 0,
            data: [0; 512],
            drive: 1,
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
