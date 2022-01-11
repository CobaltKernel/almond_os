use alloc::vec::Vec;
use bit_field::BitField;

use crate::KResult;

use super::block::Block;

pub struct Bitmap;

impl Bitmap {

    fn data_offset(block_addr: usize) -> usize {
        block_addr - 8192
    }

    fn meta_offset(block_addr: usize) -> usize {
        block_addr - 4096
    }

    fn bit_offset(block_addr: usize) -> usize {
         block_addr % 8
    }

    fn byte_offset(block_addr: usize) -> usize {
        block_addr / 8
    }

    fn block_offset(block_addr: usize) -> usize {
        block_addr / 4096
    }

    pub fn is_data_free(block_addr: usize) -> KResult<bool> {
        let offset = Self::data_offset(block_addr);
        let block = Block::read(Self::block_offset(offset) as u32)?;
        return Ok(block[Self::byte_offset(offset)].get_bit(Self::bit_offset(offset)));
    }

    pub fn is_meta_free(block_addr: usize) -> KResult<bool> {
        let offset = Self::meta_offset(block_addr);
        let block = Block::read(Self::block_offset(offset) as u32)?;
        return Ok(block[Self::byte_offset(offset)].get_bit(Self::bit_offset(offset)));
    }

    pub fn allocate_data(block_addr: usize) -> KResult<()> {
        let offset = Self::data_offset(block_addr);
        let mut block = Block::read(Self::block_offset(offset) as u32)?;
        block[Self::byte_offset(offset)].set_bit(Self::bit_offset(offset), true);
        block.write()
    }

    pub fn allocate_meta(block_addr: usize) -> KResult<()> {
        let offset = Self::meta_offset(block_addr);
        let mut block = Block::read(Self::block_offset(offset) as u32)?;
        block[Self::byte_offset(offset)].set_bit(Self::bit_offset(offset), true);
        block.write()
    }

    pub fn next_data_block() -> KResult<u32> {
        for addr in 8192..12288 {
            if Self::is_data_free(addr)? {
                return Ok(addr as u32);
            }
        }
        return Err("No Free Data Blocks");
    }

    pub fn next_meta_block() -> KResult<u32> {
        for addr in 4096..8192 {
            if Self::is_meta_free(addr)? {
                return Ok(addr as u32);
            }
        }
        return Err("No Free Data Blocks");
    }

    pub fn allocate_next_meta() -> KResult<Block> {
        Block::read(Self::next_meta_block()?)
    }

    pub fn allocate_next_data() -> KResult<Block> {
        Block::read(Self::next_data_block()?)
    }

    /// Attempt To Allocate <count> Data Blocks, Fails If No More Blocks Can Be Allocated
    pub fn allocate_multi_data(count: usize) -> KResult<Vec<Block>> {
        let mut output = Vec::new();
        for _ in 0..count {
            output.push(Self::allocate_next_data()?);
        }
        return Ok(output);
    } 
}