//! A Physically Continous Buffer

use alloc::vec;
use vec::Vec;
use crate::KResult;

/// A Continous Buffer Of Type T
#[derive(Debug)]
pub struct Buffer<T> {
    contents: Vec<T>
}

impl<T: Copy> Buffer<T> {
    /// Create A New Continous Buffer in memory, Returns Err If No Free Memory.
    pub fn new(instance: T, size: usize) -> KResult<Buffer<T>> {
        for _ in 0..1024 {
            let v = vec![instance; size];
            let start = (&v[0] as *const T) as usize;
            let end = (&v[v.len() - 1] as *const T) as usize;
            if (end - start) == size {
                return Ok(Buffer {
                    contents: v
                });
            };
            drop(v);
        };
        Err("Unable To Create A Large Enough Buffer.")
    }
}