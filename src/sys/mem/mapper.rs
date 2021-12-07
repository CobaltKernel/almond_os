//! Utilities For The [OffsetPageTable] Mapper

use core::alloc::Layout;

use bootloader::BootInfo;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::{PhysAddr, VirtAddr, structures::paging::{FrameAllocator, Mapper, OffsetPageTable, Page, PhysFrame, Size4KiB}};

use crate::sys::mem::mapper;

use super::allocator;

type TableMapper = OffsetPageTable<'static>;
static mut PHYSICAL_OFFSET: u64 = 0;

/// Initialize The Mapper
pub fn init_mapper(boot_info: &'static BootInfo) -> TableMapper {
    let offset = VirtAddr::new(boot_info.physical_memory_offset);
    unsafe {
        PHYSICAL_OFFSET = boot_info.physical_memory_offset;
        OffsetPageTable::new(super::l4_page_table_at(offset), offset)
    }
}


/// Map A Page To A Given Physical Address, Using The Given Frame Allocator.
pub unsafe fn map_to(frame_allocator: &mut impl FrameAllocator<Size4KiB>, address: PhysAddr, page: Page) {
    use x86_64::structures::paging::PageTableFlags as Flags;
    let frame = PhysFrame::containing_address(address);
    let flags = Flags::PRESENT | Flags::WRITABLE;
    let mut mapper = {
        let offset = VirtAddr::new(PHYSICAL_OFFSET);
        OffsetPageTable::new(super::l4_page_table_at(offset), offset)
    };
    let map_to_result = {
        // FIXME: this is not safe, we do it only for testing
        mapper.map_to(page, frame, flags, frame_allocator)
    };
    map_to_result.expect("map_to failed").flush();
}



