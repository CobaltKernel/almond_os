//! General File System Utilities & Traits

use alloc::string::{String, ToString};

use crate::slog;

#[derive(Debug)]
/// Represents An Object That Implements [FileIO]
pub struct FileHandle(u16);

/// Abstracts File I/O Functions
pub trait FileIO {
    /// Read Into The Buffer, Returns The Amount Read.
    fn read(&mut self, buffer: &mut [u8]) -> usize;
    /// Write From The Buffer, Returns The Amount Written.
    fn write(&mut self, buffer: &[u8]) -> usize;
    /// Set The Current Position
    fn seek(&mut self, index: usize);
    /// The File Size
    fn size(&mut self) -> usize;
    /// The Current Position
    fn pos(&mut self) -> usize;
    /// Close The File
    fn close(&mut self);

    /// Write Ptr Data To The File.
    unsafe fn write_ptr(&mut self, ptr: *const u8, len: usize) -> usize {
        let slice = core::slice::from_raw_parts(ptr, len);
        self.write(slice)
    }
}

/// Abstract File System Operations
pub trait FileSystem {
    /// Create A file. If The File Exists, Attempt To Open That File. Returns None If
    /// The File Couldn't Be Created, Or An Already Existing File Is Open In Another Process.
    fn create_file(name: String) -> Option<FileHandle>;

    /// Delete A File
    fn delete_file(handle: FileHandle);

    /// Create A Directory
    fn create_dir(name: String) -> FileHandle;
}

/// Read A String From A Slice.
pub fn read_string_utf8(data: &[u8], def: &str, offset: usize, len: usize) -> String {
    return String::from_utf8(data[offset..offset + len].to_vec()).unwrap_or(def.to_string());
}

/// Read A Nul-Terminated ASCII String From A Slice
pub fn read_asciiz(slice: &[u8], offset: usize, len: usize) -> String {
    let slice = &slice[offset..offset+len];
    let mut s = String::new();
    for byte in slice {
        if *byte == 0 { break; }
        s.push(*byte as char);
    }
    s
}


/// Read A Nul-Terminated ASCII String From A Slice
pub fn read_asciiz_octal(slice: &[u8], offset: usize, len: usize) -> usize {
    //slog!("String: {}\n", read_asciiz(slice, offset, len));
    usize::from_str_radix(&read_asciiz(slice, offset, len), 8).unwrap_or_default()
}

