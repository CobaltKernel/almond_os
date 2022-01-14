//! Read & Write To USTAR POSIX P1003.1 (1988) Files
//! [Wikipedia](https://en.wikipedia.org/wiki/Tar_(computing))

use alloc::{string::{String, ToString}, vec::{Vec}, vec};
use ata::Disk;

use crate::{sys::storage::fs_utils};

use super::ata;
#[allow(unused)]
#[derive(Debug)]
/// Implementations Of [USTAR MetaData Nodes](https://en.wikipedia.org/wiki/Tar_(computing)#Header)
pub struct MetaData {

    addr: u32,
    drive: usize,

    /// Filename - Offset: 0, Size: 100B
    name: String,

    /// Mode - Offset: 100, Size: 8
    mode: u16,

    /// Owner User ID - Offset: 108, Size: 8
    user_id: u8,

    /// Owner Group ID - Offset: 116, Size: 8
    group_id: u8,

    /// File Size - Offset: 124, Size: 12
    file_size: u32,

    /// Modification Time - Offset: 136, Size 12,
    modification_time: u32,

    /// Checksum - Offset: 148, Size: 8
    checksum: u8,

    /// File Type - Offset: 156, Size: 1,
    file_type: u8,

    /// Linked File Name - Offset: 157, Size: 100
    linked_name: String



}


impl MetaData {


    /// Load MetaData From The Drive At Address
    pub fn from(drive: usize, addr: u32) -> MetaData {
        let block = ata::read_block(drive, addr).expect("Failed To Read Disk");
        let name = fs_utils::read_asciiz(&block, 0, 100);
            Self {
                addr,
                drive,
                name,
                mode: fs_utils::read_asciiz_octal(&block, 100, 8) as u16,
                file_size: fs_utils::read_asciiz_octal(&block, 124, 12) as u32,

                checksum: 0,
                file_type: 0,
                group_id: 0,
                linked_name: String::default(),
                modification_time: 0,
                user_id: 0,
            }
    }

    /// Search For A File
    pub fn load(drive: usize, name: &str) -> Option<MetaData> {
        let mut disk = Disk::new(drive).unwrap();
        for addr in 0..disk.identify().unwrap().2 {
            let block = ata::read_block(drive, addr).expect("Disk Read Err");

            let file_name = fs_utils::read_asciiz(&block, 0, 100);
            //log!("{}\n", file_name);
            if file_name.eq(name) {
                return Some(
                    Self {
                        addr,
                        drive,
                        name: file_name,
                        mode: fs_utils::read_asciiz_octal(&block, 100, 8) as u16,
                        file_size: fs_utils::read_asciiz_octal(&block, 124, 12) as u32,

                        checksum: 0,
                        file_type: 0,
                        group_id: 0,
                        linked_name: String::default(),
                        modification_time: 0,
                        user_id: 0,
                    }
                );
            };
        }

        return None;
    } 


    /// Reads As Much Data As It Can, Returns The Amount Read
    pub fn read_data(&self, buffer: &mut [u8]) -> usize {
        let mut bytes_read: usize = 0;
        let mut block= [0; 512];
        let mut next_block: u32 = self.addr + 1;
        for idx in 0..self.file_size {
            if idx % 512 == 0 {
                block = ata::read_block(self.drive, next_block).expect("Unable To Read Disk...");
                next_block += 1;
            }

            buffer[idx as usize] = block[idx as usize % 512];
            bytes_read += 1;
        }


        return bytes_read;
    }

    /// Read The File Data Into A String
    pub fn read_string(&self) -> String {
        let mut buf= vec![0; self.file_size as usize];
        self.read_data(&mut buf);
        String::from_utf8_lossy(&buf).to_string()
    }


    /// The File Size
    pub fn file_size(&self) -> usize {
        return self.file_size as usize;
    }
}

/// List Files On Drive 1, WORK IN PROGRESS
pub fn list(drive: usize) -> Vec<String> {
    let mut disk = ata::Disk::new(drive).unwrap();
    let mut files = Vec::new();
    for address in 0..disk.identify().unwrap().2 {
        files.push(MetaData::from(drive, address).name);
    }

    files
}

pub fn load_bytes(path: &str, buffer: &mut [u8]) -> usize {
    if let Some(md) = MetaData::load(1, path) {
        return md.read_data(buffer);
    } else {
        return 0;
    }
}