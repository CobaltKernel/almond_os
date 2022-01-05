pub mod null_allocator;
use core::alloc::{GlobalAlloc, Layout};

pub mod bump;
pub mod linked_list;

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 1024 * 1024 * 2; // 1MiB

#[cfg_attr(feature = "list_allocator", global_allocator)]
static LINKED_LIST_ALLOCATOR: Locked<linked_list::LinkedListAllocator> =
    Locked::new(linked_list::LinkedListAllocator::new());

#[cfg_attr(feature = "bump_allocator", global_allocator)]
static BUMP: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

#[derive(Debug, Clone, Copy)]
pub struct MemoryStats {
    pub allocated: usize,
}

use bootloader::BootInfo;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

use crate::{print, sys::terminal::Spinner, KResult, Locked};

use self::bump::BumpAllocator;

use super::{frame_allocator::BootInfoFrameAllocator, mapper::init_mapper};

fn init_kheap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let mut spinner = Spinner::new();
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };
    print!("\n");
    let mut i = 0;
    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
        i += 1;
        if i % 64 == 0 && i > 0 {
            spinner.update();
            crate::print!("Initing Kernel Heap - {}  \r", spinner.glyph());
        }
    }
    print!("\n");

    Ok(())
}

/// Initialize The Memory Subsystem.
pub fn initialize(info: &'static BootInfo) -> KResult<()> {
    let mut mapper = init_mapper(info);
    unsafe {
        let mut frame_alloc: BootInfoFrameAllocator = BootInfoFrameAllocator::from(info);
        init_kheap(&mut mapper, &mut frame_alloc).expect("Failed To Init Kernel Heap.");
        LINKED_LIST_ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
        BUMP.lock().init(HEAP_START, HEAP_START + HEAP_SIZE);
    }

    Ok(())
}

/// Align the given address `addr` upwards to alignment `align`.
///
/// Requires that `align` is a power of two.
fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

#[cfg(feature = "bump_allocator")]
fn current_allocator() -> &'static impl GlobalAlloc {
    &BUMP
}

#[cfg(feature = "list_allocator")]
fn current_allocator() -> &'static impl GlobalAlloc {
    &LINKED_LIST_ALLOCATOR
}

/// Allocate Memory On The Kernel Heap.
pub unsafe fn malloc(layout: Layout) -> *mut u8 {
    let allocator = current_allocator();
    allocator.alloc_zeroed(layout)
}

/// Free Memory On The Kernel Heap.
pub unsafe fn free(ptr: *mut u8, layout: Layout) {
    let allocator = current_allocator();
    allocator.dealloc(ptr, layout)
}
