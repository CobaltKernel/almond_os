//! Data Bitmap Routines

use bit_field::BitField;

use crate::sys::storage::ata;

use super::partition::{DATA_SIZE, DATA_START};

/// Data Bitmap Functions.
#[derive(Debug)]
pub struct DataBitmap;

impl DataBitmap {
    #[inline(always)]
    fn bitmap(block: u32) -> u32 {
        assert!(block >= DATA_START, "Data Block Out Of Range.");
        let block = block - DATA_START;
        block / 4069
    }
    #[inline(always)]
    fn offset(block: u32) -> u32 {
        assert!(block >= DATA_START, "Data Block Out Of Range.");
        let block = block - DATA_START;
        block / 8
    }

    #[inline(always)]
    fn bit(block: u32) -> u32 {
        assert!(block >= DATA_START, "Data Block Out Of Range.");
        let block = block - DATA_START;
        block & 7
    }

    /// Check Whether Or Not The Given [DataBlock] Address Is Free.
    pub fn is_free(drive: usize, block: u32) -> bool {
        let mut bitmap = [0; 512];
        ata::read(drive, Self::bitmap(block), &mut bitmap).expect("Disk Read Error");
        return !bitmap[Self::offset(block) as usize].get_bit(Self::bit(block) as usize);
    }

    /// Check Whether Or Not The Given [DataBlock] Address Used.
    pub fn is_used(drive: usize, block: u32) -> bool {
        let mut bitmap = [0; 512];
        ata::read(drive, Self::bitmap(block), &mut bitmap).expect("Disk Read Error");
        return bitmap[Self::offset(block) as usize].get_bit(Self::bit(block) as usize);
    }

    /// Allocate A DataBlock Address
    pub fn allocate(drive: usize, block: u32) {
        let mut bitmap = [0; 512];
        ata::read(drive, Self::bitmap(block), &mut bitmap).expect("Disk Read Error");
        bitmap[Self::offset(block) as usize].set_bit(Self::bit(block) as usize, true);
        ata::write(drive, Self::bitmap(block), &bitmap).expect("MKA");
    }

    /// Free A DataBlock Address
    pub fn free(drive: usize, block: u32) {
        let mut bitmap = [0; 512];
        ata::read(drive, Self::bitmap(block), &mut bitmap).expect("Disk Read Error");
        bitmap[Self::offset(block) as usize].set_bit(Self::bit(block) as usize, false);
        ata::write(drive, Self::bitmap(block), &bitmap).expect("Disk Write Error");
    }

    /// Get The Next Free DataBlock
    pub fn next_free(drive: usize) -> Option<u32> {
        for block in DATA_START..(DATA_START + DATA_SIZE as u32) {
            if Self::is_free(drive, block) {
                return Some(block);
            } else {
            }
        }
        return None;
    }
}
