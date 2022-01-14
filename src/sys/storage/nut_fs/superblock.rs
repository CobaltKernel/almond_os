//! The NutFS Superblock

use alloc::{borrow::ToOwned, string::String};

use crate::{
    log,
    sys::{storage::ata},
    KResult,
};

const MAGIC: &'static str = "NUTFS";

/// See NUTFS Layout Spreadsheet
#[derive(Debug)]
pub struct SuperBlock {
    volume_name: String,
    volume_size: u32,
    volume_start: u32,
    data_size: u8,
    data_start: u32,
    meta_size: u8,
    meta_start: u32,
}

impl SuperBlock {
    /// Converts A Byte Slice Into A Superblock. Returns [Option::None] If The Data is Malformed.
    pub fn new(data: &[u8]) -> Option<SuperBlock> {
        log!(
            "Magic ID: {} - Needed: {}",
            String::from_utf8(data[34..39].to_vec())
                .unwrap_or_default()
                .trim_end()
                .to_owned(),
            MAGIC
        );
        if String::from_utf8(data[34..39].to_vec()).unwrap_or_default() != MAGIC {
            return None;
        };

        return Some(Self {
            volume_name: String::from_utf8(data[0..16].to_vec())
                .unwrap_or_default()
                .trim_end()
                .to_owned(),
            volume_size: u32::from_be_bytes((&data[16..20]).try_into().unwrap()),
            volume_start: u32::from_be_bytes((&data[20..24]).try_into().unwrap()),
            data_size: u8::from_be_bytes((&data[24..25]).try_into().unwrap()),
            data_start: u32::from_be_bytes((&data[25..29]).try_into().unwrap()),
            meta_size: u8::from_be_bytes((&data[29..30]).try_into().unwrap()),
            meta_start: u32::from_be_bytes((&data[30..34]).try_into().unwrap()),
        });
    }

    /// Load The Superblock from Drive.
    pub fn load(drive: usize) -> KResult<Self> {
        let data = ata::read_block(drive, 0)?;

        if let Some(block) = SuperBlock::new(&data) {
            Ok(block)
        } else {
            Err("Failed To Create SuperBlock")
        }
    }

    /// Create A New Superblock
    pub fn create(
        volume_name: &str,
        volume_start: u32,
        volume_size: u32,
        data_start: u32,
        data_size: u8,
        meta_start: u32,
        meta_size: u8,
    ) -> SuperBlock {
        Self {
            data_size,
            data_start,
            meta_size,
            meta_start,
            volume_name: volume_name.to_owned(),
            volume_size,
            volume_start,
        }
    }

    /// Writes The Superblock To The Drive.
    pub fn write(&self, drive: usize, addr: u32) -> KResult<()> {
        let mut buffer = [0; 512];
        for i in 0..self.volume_name.len() {
            buffer[i] = self.volume_name.as_bytes()[i];
        }

        for i in self.volume_name.len()..16 {
            buffer[i as usize] = b' ';
        }

        buffer[16..20].copy_from_slice(&self.volume_size.to_be_bytes());
        buffer[20..24].copy_from_slice(&self.volume_start.to_be_bytes());
        buffer[24..25].copy_from_slice(&self.data_size.to_be_bytes());
        buffer[25..29].copy_from_slice(&self.data_start.to_be_bytes());
        buffer[29..30].copy_from_slice(&self.meta_size.to_be_bytes());
        buffer[30..34].copy_from_slice(&self.meta_start.to_be_bytes());

        buffer[34..39].copy_from_slice(MAGIC.as_bytes());

        ata::write(drive, addr, &buffer)?;

        Ok(())
    }
}
