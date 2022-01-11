use core::{ops::{Index, IndexMut, Range, RangeInclusive, RangeFrom, RangeFull}};

use alloc::string::String;

use crate::{vfs::{self, BlockDeviceIO, BLOCK_SIZE}, KResult};

use super::filesystem::BLOCKS_IN_BITMAP;

#[derive(Debug, Clone, Copy)]
pub struct Block {
    addr: u32,
    data: [u8; 512],
}

impl Block {
    pub fn read(addr: u32) -> KResult<Block> {
        if let Some(dev) = &*vfs::device() {
            return Ok(Self {
                addr,
                data: dev.read(addr as usize)?,
            });
        } else {
            return Err("Device Not Mounted.");
        }
    }

    pub fn write(&self) -> KResult<()> {
        if let Some(dev) = &mut *vfs::device() {
            dev.write(self.addr as usize, &self.data)?;
            return Ok(());
        } else {
            return Err("Device Not Mounted.");
        }
    }


    pub fn read_u8(&self, index: usize) -> u8 {
        self[index]
    }

    pub fn read_u16_be(&self, index: usize) -> u16 {
        let slice = &self[index..=index+1];
        let mut bytes = [0; 2];
        for i in 0..bytes.len() {
            bytes[i] = slice[i];
        }
        u16::from_be_bytes(bytes)
    }

    pub fn read_u32_be(&self, index: usize) -> u32 {
        let slice = &self[index..index+4];
        let mut bytes = [0; 4];
        for i in 0..bytes.len() {
            bytes[i] = slice[i];
        }
        u32::from_be_bytes(bytes)
    }

    pub fn read_u64_be(&self, index: usize) -> u64 {
        let slice = &self[index..index+8];
        let mut bytes = [0; 8];
        for i in 0..bytes.len() {
            bytes[i] = slice[i];
        }
        u64::from_be_bytes(bytes)
    }

    pub fn read_asciiz(&self, index: usize) -> String {
        let mut text = String::new();
        for pos in index..BLOCK_SIZE {
            if self[pos] > 0 {
                text.push(self[pos] as char);
            } else {
                return text;
            }
        }
        return text;
    }

    pub fn read_u16s(&self, index: usize, buffer: &mut [u16]) -> usize {
        for i in 0..buffer.len() {
            if i + index > BLOCK_SIZE {
                return i;
            }

            buffer[i] = self.read_u16_be(index + i * 2);

        }
        return buffer.len();
    }
}

impl Index<usize> for Block {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Block {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Index<Range<usize>> for Block {
    type Output = [u8];
    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<Range<usize>> for Block {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Index<RangeInclusive<usize>> for Block {
    type Output = [u8];
    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<RangeInclusive<usize>> for Block {
    fn index_mut(&mut self, index: RangeInclusive<usize>) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Index<RangeFrom<usize>> for Block {
    type Output = [u8];
    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<RangeFrom<usize>> for Block {
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl Index<RangeFull> for Block {
    type Output = [u8];
    fn index(&self, index: RangeFull) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<RangeFull> for Block {
    fn index_mut(&mut self, index: RangeFull) -> &mut Self::Output {
        &mut self.data[index]
    }
}



