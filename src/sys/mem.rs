//! [UNSTABLE]
//! Utility Functions For Managing Memory & The Kernel heap.
use core::alloc::Layout;

use bootloader::BootInfo;
use x86_64::{structures::paging::PageTable, VirtAddr};

use crate::KResult;

mod allocator;
pub mod frame_allocator;
pub mod mapper;

pub mod buffer;
pub mod ringbuffer;

/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
pub(self) unsafe fn l4_page_table_at(offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}

/// Initialize The Memory Subsystem.
pub fn initialize(info: &'static BootInfo) -> KResult<()> {
    allocator::initialize(info)
}

/// Allocate Memory On The Kernel Heap.
pub unsafe fn malloc(layout: Layout) -> *mut u8 {
    allocator::malloc(layout)
}

/// Allocate Memory On The Kernel Heap.
pub unsafe fn free(ptr: *mut u8, layout: Layout) {
    allocator::free(ptr, layout);
}
