//! TODO

use alloc::string::*;

use crate::{sys::storage::ata, KResult};

use super::{
    datablock::DataBlock,
    meta_bitmap::{self},
};

/// Encodes File Type Information
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileType {
    /// MetaData Block Is Free
    Free = 0,
    /// Represents A File
    File = 1,
    /// Represents A Directory
    Directory = 2,
    /// Represents A Character Device
    CharDev = 3,
    /// Represents A Block Device
    BlockDev = 4,
}

impl FileType {
    /// Converts a [u8] To a [FileType]
    pub fn from_u8(byte: u8) -> Self {
        match byte {
            0 => Self::Free,
            1 => Self::File,
            2 => Self::Directory,
            3 => Self::CharDev,
            4 => Self::BlockDev,
            _ => Self::Free,
        }
    }
}

/// File Meta Data
#[derive(Debug, Clone)]
pub struct MetaData {
    addr: u32,
    name: String,
    file_type: FileType,
    size: u32,
    start: u32,
}

impl MetaData {
    /// This MetaData's LBA
    pub fn addr(&self) -> u32 {
        self.addr
    }

    /// Allocate A New MetaData Block
    pub fn allocate(
        drive: usize,
        name: &str,
        file_type: FileType,
        data_start: u32,
    ) -> KResult<MetaData> {
        if let Some(block) = meta_bitmap::MetaBitmap::next_free(drive) {
            Ok(MetaData {
                addr: block,
                file_type,
                name: name.to_string(),
                size: 0,
                start: data_start,
            })
        } else {
            Err("Unable To Create File.")
        }
    }

    /// Load MetaData From A Drive.
    pub fn load(drive: usize, index: u32) -> KResult<MetaData> {
        let mut data: [u8; 512] = [0; 512];
        ata::read(drive, index, &mut data)?;
        let name = String::from_utf8(data[0..256].to_vec()).unwrap_or("UNK".to_string());
        let file_type = FileType::from_u8(u8::from_be_bytes((&data[256..257]).try_into().unwrap()));
        let size = u32::from_be_bytes((&data[257..261]).try_into().unwrap());
        let start = u32::from_be_bytes((&data[261..265]).try_into().unwrap());

        Ok(Self {
            addr: index,
            file_type,
            name,
            size,
            start,
        })
    }

    /// Creates New MetaData
    pub fn new(index: u32, name: &str, file_type: FileType, start: u32) -> Self {
        Self {
            addr: index,
            file_type,
            name: name.to_string(),
            size: 0,
            start,
        }
    }

    /// Write To Drive At Index
    pub fn save(&self, drive: usize) -> KResult<()> {
        let mut data: [u8; 512] = [0x20; 512];
        data[0..self.name.as_bytes().len()].copy_from_slice(self.name.as_bytes());
        data[256..257].copy_from_slice(&(self.file_type as u8).to_be_bytes());
        data[257..261].copy_from_slice(&self.size.to_be_bytes());
        data[261..265].copy_from_slice(&self.start.to_be_bytes());

        ata::write(drive, self.addr, &data)?;

        Ok(())
    }

    /// Set The File Size
    pub fn set_size(&mut self, size: u32) {
        self.size = size;
    }

    /// Set Head DataBlock
    pub fn set_data(&mut self, block: &DataBlock) {
        self.start = block.addr()
    }
}
