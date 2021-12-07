//! The NutFS Implementation

use core::alloc::{Layout};

use alloc::vec::{Vec};
use alloc::vec;

use crate::sys::mem;
use crate::sys::terminal::Spinner;
use crate::{KResult, log, slog};

use self::datablock::DataBlock;
use self::metablock::MetaData;
use self::partition::{KERNEL_START, KERNEL_SIZE};

use super::ata;

pub mod datablock;
pub mod metablock;
pub mod superblock;

pub mod partition;
pub mod meta_bitmap;
pub mod data_bitmap;


/// Write A Block Of Data At Index
/// Marked As Unsafe As The Caller MUST Guarantee That The Given Block Index
/// Is Unused
pub unsafe fn write_data(drive: usize, index: u32, data: &[u8]) -> KResult<DataBlock> {
    let head = DataBlock::create(index);
    let mut index = index;
    let mut current = head;
    for (i, chunk) in data.chunks(508).into_iter().enumerate() {
        slog!("== Saving Chunk #{:04} == \n", i);
        for (ind, byte) in chunk.iter().enumerate() {
            current.data_mut()[ind] = *byte;
        }
        if chunk.len() >= 508 {
            index += 1;
            current.set_next(index);
        }
        current.write(drive)?;
        current = DataBlock::create(index);
        slog!("{}\n", "=".repeat(64));
    }
    //head.write(drive)?;
    return Ok(head);
}

/// Write Some Data To Disk, Allocating Blocks As Needed.
pub fn alloc_data(drive: usize, data: &[u8]) -> KResult<DataBlock> {
    let head = DataBlock::allocate(drive)?;
    let mut current = head;
    let mut last = head;
    for (i, chunk) in data.chunks(508).into_iter().enumerate() {

        slog!("== Saving Chunk #{:04} == \n", i);
        for (ind, byte) in chunk.iter().enumerate() {
            current.data_mut()[ind] = *byte;
        }
        if chunk.len() >= 508 {
            last = current.clone();
            current = DataBlock::allocate(drive)?;
            last.set_next(current.addr());
        }
        last.write(drive)?;
        slog!("{}\n", "=".repeat(64));
    }
    //head.write(drive)?;
    return Ok(head);
}

/// Read Data from Disk, Put It Into buffer.
/// Returns (The Index of The Last Block Read, The Position Within That Block, The Position Within The Buffer)
pub fn read(drive: usize, index: u32, buffer: &mut [u8]) -> KResult<(u32, usize, usize)> {
    let mut blocks = Vec::new();
    let mut blocks_used = 0;
    datablock::read_blocks(drive, index, &mut blocks)?;
    let mut pos = 0;
    for block in blocks {
        for (_i, byte) in block.data().iter().enumerate() {
            if pos < buffer.len() {
                buffer[pos] = *byte;
                pos += 1;
            }
        }

        blocks_used += 1;
    }

    Ok((index + blocks_used - 1, pos % 508, pos))
}

/// Copy Pointer Data Onto Disk. 
/// UNSAFE BECAUSE LEN MUST BE EXACTLY EQUAL TO THE UNDERLYING STRUCTURE'S SIZE.
pub unsafe fn write_const_ptr(drive: usize, index: u32, ptr: *const u8, len: usize) -> KResult<DataBlock> {
    let slice = core::slice::from_raw_parts(ptr, len);
    write_data(drive, index, slice)
}

/// Copy Pointer Data Onto Disk. 
/// UNSAFE BECAUSE LEN MUST BE EXACTLY EQUAL TO THE UNDERLYING STRUCTURE'S SIZE.
pub unsafe fn write_mut_ptr(drive: usize, index: u32, ptr: *mut u8, len: usize) -> KResult<DataBlock> {
    let slice = core::slice::from_raw_parts(ptr, len);
    write_data(drive, index, slice)
}


/// Copy Pointer Data Onto Disk. 
/// UNSAFE BECAUSE LEN MUST BE EXACTLY EQUAL TO THE UNDERLYING STRUCTURE'S SIZE AND
/// ALIGN MUST BE A POWER OF TWO.
pub unsafe fn read_ptr_mut<T>(drive: usize, index: u32, len: usize, align: usize) -> KResult<*mut T> {
    let mut buffer = vec![0; len];
    read(drive, index, &mut buffer)?;
    let ptr = mem::malloc(Layout::from_size_align_unchecked(len, align)) as *mut u8;
    for offset in 0..len {
        ptr.offset(offset as isize).write_volatile(buffer[offset]); 
    }
    Ok(ptr as *mut T)
}


/// Reads Meta Data From Drive At Index
pub fn read_metadata(drive: usize, index: u32) -> KResult<MetaData> {
    MetaData::load(drive, index)
}
 

/// Create A New File (DO NOT RUN).
pub fn create_file(name: &str) -> KResult<MetaData> {
    todo!()
}

fn allocate_metadata() -> Option<u32> {
    todo!()
}

fn allocate_datablock() -> Option<u32> {
    todo!()
}

const META_BITMAP_START: u32 = 4096;
const DATA_BITMAP_START: u32 = 2048;
const BLOCK_SIZE: u32 = 512;
const BLOCKS_PER_BITMAP: u32 = 8 * BLOCK_SIZE;

struct Bitmap;


impl Bitmap {
    pub fn is_meta_free(drive: usize, address: u32) -> bool {
        let bitmap = ();
        let offset =  address / 8;
        let bit =     address % 8;

        let data = [0; 512];

            // bitmap = (meta_addr - meta_data_start) / 4096

            todo!()
    }
}


/// Copies The Bootloader & Kernel Onto The 2nd Drive.
pub unsafe fn install() -> KResult<()> {
    ata::copy(1,0, KERNEL_START as u32, ata::Disk::new(0).unwrap().identify().unwrap().2 as usize)?;
    Ok(())
}


/// Formats The Drive. Starts From Sector 0, Up to Size.
pub unsafe fn format(drive: usize) -> KResult<()> {
    let mut spinner = Spinner::new();
    let size = ata::Disk::new(drive).unwrap().identify().unwrap().2 as usize;
    for i in 0..size {
        ata::write(drive, i as u32, &[0; 512])?;

        if i % 128 == 0 {
            log!("Formatting Sectors... {} - ({:05}/{:05})\r", spinner.glyph(), i, size);
            spinner.update();
        }
    }

    log!("Formatted Sectors...                     \n");
    Ok(())
}