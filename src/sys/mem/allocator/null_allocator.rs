use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

use crate::print;

pub struct NullAllocator;

unsafe impl GlobalAlloc for NullAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        print!("Attempting To Allocate {} Bytes", _layout.size());
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("dealloc should be never called")
    }
}
