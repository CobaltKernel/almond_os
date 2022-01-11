use alloc::string::String;

use crate::KResult;

use super::{block::Block, filesystem::METADATA_START};

pub struct MetaDataBlock {
    index: usize,
    /// File/Dir Name - Pos: 0, Size: 256 (NT)
    name: String, 

    /// File Size - Pos: 256, Size: 4
    size: u32,
}

impl MetaDataBlock {
    pub fn read(index: usize) -> KResult<MetaDataBlock> {

        let block = Block::read((index + METADATA_START) as u32)?;

        Ok(
            Self {
                index,
                name: block.read_asciiz(0),
                size: block.read_u32_be(256),
            }
        )
    }
}