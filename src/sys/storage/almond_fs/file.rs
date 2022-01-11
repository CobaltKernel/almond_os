use alloc::vec::Vec;

use crate::vfs::FileIO;

#[derive(Debug)]
pub struct File {
    _private: (),
    data: Vec<u8>,
    pos: isize,
}

impl FileIO for File {
    fn pos(&self) -> usize {
        todo!()
    }

    fn seek(&mut self, amnt: isize) -> usize {
        self.pos += amnt;
        self.pos = self.pos.clamp(0, self.data.len() as isize);
        self.pos as usize
    }

    fn read(&mut self, buffer: &mut [u8]) -> usize {
        let mut count = 0;
        for index in 0..buffer.len() {
            if (self.pos as usize) < self.data.len() {
                buffer[index] = self.data[self.pos.clamp(0,self.data.len() as isize) as usize];
                count += 1;
            }
        }
        return count;
    }

    fn write(&mut self, buffer: &[u8]) -> usize {
        let mut count = 0;
        let size = self.data.len() as isize;
        for (_, byte) in buffer.iter().enumerate() {
            if (self.pos as usize) < self.data.len() {
                self.data[self.pos.clamp(0,size) as usize] = *byte;
                count += 1;
            }
        }
        return count;
    }

    fn append(&mut self, buffer: &[u8]) -> usize {
        for byte in buffer {
            self.data.push(*byte);
        }
        buffer.len()
    }

    fn size(&self) -> usize {
        return self.data.len();
    }

    fn close(&mut self) {
        todo!()
    }
}