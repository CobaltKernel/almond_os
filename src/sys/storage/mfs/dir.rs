use alloc::string::String;

use super::{super_block::SuperBlock, dir_entry::DirEntry, read_dir::ReadDir, api::{realpath, dirname, filename}, linked_block::LinkedBlock, FileType, bitmap_block::BitmapBlock};

#[derive(Debug, Clone, Copy)]
pub struct Dir {
    addr: u32,
}


impl From<DirEntry> for Dir {
    fn from(entry: DirEntry) -> Self {
        Self { addr: entry.addr() }
    }
}

impl Dir {
    pub fn root() -> Self {
        Self {addr: SuperBlock::read().expect("SBRF").data_area()}
    }

    pub fn open(pathname: &str) -> Option<Self> {
        if !crate::vfs::is_mounted() {
            return None;
        }

        let mut dir = Dir::root();
        let pathname = realpath(pathname);

        if pathname == "/" {
            return Some(dir);
        }

        for name in pathname.trim_start_matches('/').split('/') {
            match dir.find(name) {
                Some(dir_entry) => {
                    if dir_entry.is_dir() {
                        dir = dir_entry.into()
                    } else {
                        return None;
                    }
                },
                None => {
                    return None
                },
            }
        }
        Some(dir)
    }

    pub fn addr(&self) -> u32 {
        self.addr
    }

    pub fn find(&self, name: &str) -> Option<DirEntry> {
        for entry in self.entries() {
            if entry.name() == name {
                return Some(entry);
            }
        }
        None
    }

    pub fn entries(&self) -> ReadDir {
        ReadDir::from(*self)
    }

     // TODO: return a Result
     pub fn create_file(&self, name: &str) -> Option<DirEntry> {
        self.create_entry(FileType::File, name)
    }

    pub fn create_dir(&self, name: &str) -> Option<DirEntry> {
        self.create_entry(FileType::Dir, name)
    }

    pub fn create_device(&self, name: &str) -> Option<DirEntry> {
        self.create_entry(FileType::Device, name)
    }

    fn create_entry(&self, kind: FileType, name: &str) -> Option<DirEntry> {
        if self.find(name).is_some() {
            return None;
        }

        // Read the whole dir to add an entry at the end
        let mut entries = self.entries();
        while entries.next().is_some() {}

        // Allocate a new block for the dir if no space left for adding the new entry
        let space_left = entries.block.data().len() - entries.block_data_offset();
        let entry_len = DirEntry::empty_len() + name.len();
        if entry_len > space_left {
            match entries.block.alloc_next() {
                None => return None, // Disk is full
                Some(new_block) => {
                    entries.block = new_block;
                    entries.block_data_offset = 0;
                },
            }
        }

        // Create a new entry
        let entry_block = LinkedBlock::alloc().unwrap();
        let entry_kind = kind as u8;
        let entry_addr = entry_block.addr();
        let entry_size = 0u32;
        let entry_time = 0u64;
        let entry_name = truncate(name, u8::MAX as usize);
        let n = entry_name.len();
        let i = entries.block_data_offset();
        let data = entries.block.data_mut();

        data[i] = entry_kind;
        data[(i + 1)..(i + 5)].clone_from_slice(&entry_addr.to_be_bytes());
        data[(i + 5)..(i + 9)].clone_from_slice(&entry_size.to_be_bytes());
        data[(i + 9)..(i + 17)].clone_from_slice(&entry_time.to_be_bytes());
        data[i + 17] = n as u8;
        data[(i + 18)..(i + 18 + n)].clone_from_slice(entry_name.as_bytes());

        entries.block.write();

        Some(DirEntry::new(*self, kind, entry_addr, entry_size, entry_time, &entry_name))
    }

    pub fn update_entry(&mut self, name: &str, size: u32) {
        let time = 0u64;
        let mut entries = self.entries();
        for entry in &mut entries {
            if entry.name() == name {
                let i = entries.block_data_offset() - entry.len();
                let data = entries.block.data_mut();
                data[(i + 5)..(i + 9)].clone_from_slice(&size.to_be_bytes());
                data[(i + 9)..(i + 17)].clone_from_slice(&time.to_be_bytes());
                entries.block.write();
                break;
            }
        }
    }

    pub fn delete(pathname: &str) -> Result<(), ()> {
        let pathname = realpath(pathname);
        let dirname = dirname(&pathname);
        let filename = filename(&pathname);
        if let Some(mut dir) = Dir::open(dirname) {
            dir.delete_entry(filename)
        } else {
            Err(())
        }
    }

    // Deleting an entry is done by setting the entry address to 0
    // TODO: If the entry is a directory, remove its entries recursively
    pub fn delete_entry(&mut self, name: &str) -> Result<(), ()> {
        let mut entries = self.entries();
        for entry in &mut entries {
            if entry.name() == name {
                // Zeroing entry addr
                let i = entries.block_data_offset() - entry.len();
                let data = entries.block.data_mut();
                data[i + 1] = 0;
                data[i + 2] = 0;
                data[i + 3] = 0;
                data[i + 4] = 0;
                entries.block.write();

                // Freeing entry blocks
                let mut entry_block = LinkedBlock::read(entry.addr());
                loop {
                    BitmapBlock::free(entry_block.addr());
                    match entry_block.next() {
                        Some(next_block) => entry_block = next_block,
                        None => break,
                    }
                }

                return Ok(());
            }
        }
        Err(())
    }

    

}

// Truncate to the given number of bytes at most while respecting char boundaries
fn truncate(s: &str, max: usize) -> String {
    s.char_indices().take_while(|(i, _)| *i <= max).map(|(_, c)| c).collect()
}