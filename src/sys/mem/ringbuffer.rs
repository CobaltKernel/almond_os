//! No Allocating Ring Buffer

use crate::KResult;

/// A [RingBuffer] Of 256 Elements
pub type RingBuffer256<T> = RingBuffer<T, 256>;

/// Zero Allocation Ringbuffer
#[derive(Debug)]
pub struct RingBuffer<T: Copy + Default, const N: usize> {
    read_ptr: usize,
    write_ptr: usize,
    buf: [T; N],
}

impl<T: Copy + Default, const N: usize> RingBuffer<T, N> {
    /// Create A New RingBuffer
    pub fn new() -> Self {
        Self {
            read_ptr: 0,
            write_ptr: 1,
            buf: [Default::default(); N],
        }
    }

    /// Reads From The Buffer Returning None If read_ptr is Equal To Write Ptr
    pub fn read(&mut self) -> Option<T> {
        if self.read_ptr != self.write_ptr {
            let res = Some(self.buf[self.read_ptr]);
            self.read_ptr += 1;
            self.read_ptr %= self.buf.len();
            return res;
        } else {
            return None;
        }
    }

    /// Write From The Buffer, Returns Err If The Write Would Corrupt Data Yet To Be Read
    pub fn write(&mut self, value: T) -> KResult<()> {
        if self.read_ptr != self.write_ptr {
            self.buf[self.write_ptr] = value;
            self.write_ptr += 1;
            self.write_ptr %= self.buf.len();
            return Ok(());
        } else {
            return Err("RingBuffer is full");
        }
    }

    /// Checks Whether The Buffer Is 'Full'
    pub fn is_full(&self) -> bool {
        self.read_ptr == self.write_ptr
    }

    /// Checks Whether The Buffer Is 'Empty'
    pub fn is_empty(&self) -> bool {
        self.read_ptr == self.write_ptr
    }
}
