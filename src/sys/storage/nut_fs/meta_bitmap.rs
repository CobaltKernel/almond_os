//! Meta Bitmap Rountines.

use bit_field::BitField;

use crate::sys::storage::{ata, nut_fs::partition::META_START};

use super::partition::META_SIZE;

/// Ties Together Functions For Reading & Writing To Meta Bitmap Blocks;
#[derive(Debug)]
pub struct MetaBitmap;

impl MetaBitmap {
    #[inline(always)]
    fn bitmap(block: u32) -> u32 {
        assert!(block >= META_START, "Data Block Out Of Range.");
        let block = block - META_START;
        block / 4069
    }
    #[inline(always)]
    fn offset(block: u32) -> u32 {
        assert!(block >= META_START, "Data Block Out Of Range.");
        let block = block - META_START;
        block / 8
    }

    #[inline(always)]
    fn bit(block: u32) -> u32 {
        assert!(block >= META_START, "Data Block Out Of Range.");
        let block = block - META_START;
        block & 7
    }

    /// Check Whether Or Not The Given [MetaData] Address Is Free.
    pub fn is_free(drive: usize, block: u32) -> bool {
        let mut bitmap = [0; 512];
        ata::read(drive, Self::bitmap(block), &mut bitmap).expect("Disk Read Error");
        return !bitmap[Self::offset(block) as usize].get_bit(Self::bit(block) as usize);
    }

    /// Check Whether Or Not The Given [MetaData] Address Used.
    pub fn is_used(drive: usize, block: u32) -> bool {
        let mut bitmap = [0; 512];
        ata::read(drive, Self::bitmap(block), &mut bitmap).expect("Disk Read Error");
        return bitmap[Self::offset(block) as usize].get_bit(Self::bit(block) as usize);
    }

    /// Allocate A MetaData Address
    pub fn allocate(drive: usize, block: u32) {
        let mut bitmap = [0; 512];
        ata::read(drive, Self::bitmap(block), &mut bitmap).expect("Disk Read Error");
        bitmap[Self::offset(block) as usize].set_bit(Self::bit(block) as usize, true);
        ata::write(drive, Self::bitmap(block), &bitmap).expect("Failed To Write Bitmap");
    }

    /// Free A MetaData Address
    pub fn free(drive: usize, block: u32) {
        let mut bitmap = [0; 512];
        ata::read(drive, Self::bitmap(block), &mut bitmap).expect("Disk Read Error");
        bitmap[Self::offset(block) as usize].set_bit(Self::bit(block) as usize, false);
        ata::write(drive, Self::bitmap(block), &bitmap).expect("Disk Write Error");
    }

    /// Get The Next Free MetaData
    pub fn next_free(drive: usize) -> Option<u32> {
        for block in META_START..(META_START + META_SIZE as u32) {
            if Self::is_free(drive, block) {
                return Some(block);
            } else {
            }
        }
        return None;
    }
}
