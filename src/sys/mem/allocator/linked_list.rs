use core::alloc::{GlobalAlloc, Layout};

use crate::{Locked, sys::{mem::allocator::align_up, timer::sleep_ticks}};

use super::alloc_error_handler;

pub struct ListNode {
    pub size: usize,
    pub next: Option<&'static mut ListNode>,
    pub prev: Option<&'static mut ListNode>,
}

impl ListNode {
    pub const fn new(size: usize) -> ListNode {
        ListNode { size, next: None, prev: None}
    }

    pub fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    pub fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }

}

pub struct LinkedListAllocator {
    head: ListNode,
}

impl LinkedListAllocator {
    pub const fn new() -> Self {
        let node = ListNode::new(0);
        Self {
            head: node,
        }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.add_free_region(heap_start, heap_size);
    }

    pub unsafe fn add_free_region(&mut self, addr: usize, size: usize) {
        assert_eq!(align_up(addr, core::mem::align_of::<ListNode>()), addr);
        assert!(size >= core::mem::size_of::<ListNode>());

        // Create A New Node Of The Given Size
        let mut node = ListNode::new(size);
        // Insert the Node into the list between the head & the second Node.
        node.next = self.head.next.take();
        // Set The Node's Prev Pointer To The Head Node
        // TODO(George): Work Out How The FUCK To Do This Whilst Keeping Rust Happy. FUCK.
        // Get The Raw Node Pointer From the given Address
        let node_ptr = addr as *mut ListNode;
        // Write The Node Data Into Memory
        node_ptr.write(node);
        // Set The Head's Next To Point To the written Node.
        self.head.next = Some(&mut *node_ptr);
        
        


    }

    fn find_region(&mut self, size: usize, align: usize) -> Option<(&'static mut ListNode, usize)> {
        let mut current = &mut self.head;

        while let Some(ref mut region) = current.next {
            if let Ok(alloc_start) = Self::alloc_from_region(&region, size, align) {
                let next = region.next.take();
                let ret = Some((current.next.take().unwrap(), alloc_start));
                current.next = next;
                return ret;
            } else {
                current = current.next.as_mut().unwrap();
            }
        }
        
        None
    }

    /// Adjust the given layout so that the resulting allocated memory
    /// region is also capable of storing a `ListNode`.
    ///
    /// Returns the adjusted size and alignment as a (size, align) tuple.
    fn size_align(layout: Layout) -> (usize, usize) {
        let layout = layout
            .align_to(core::mem::align_of::<ListNode>())
            .expect("adjusting alignment failed")
            .pad_to_align();
        let size = layout.size().max(core::mem::size_of::<ListNode>());
        (size, layout.align())
    }

    /// Try to use the given region for an allocation with given size and
    /// alignment.
    ///
    /// Returns the allocation start address on success.
    fn alloc_from_region(region: &ListNode, size: usize, align: usize)
        -> Result<usize, ()>
    {
        let alloc_start = align_up(region.start_addr(), align);
        let alloc_end = alloc_start.checked_add(size).ok_or(())?;

        if alloc_end > region.end_addr() {
            // region too small
            return Err(());
        }

        let excess_size = region.end_addr() - alloc_end;
        if excess_size > 0 && excess_size < core::mem::size_of::<ListNode>() {
            // rest of region too small to hold a ListNode (required because the
            // allocation splits the region in a used and a free part)
            return Err(());
        }

        // region suitable for allocation
        Ok(alloc_start)
    }

}


unsafe impl GlobalAlloc for Locked<LinkedListAllocator> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
         // perform layout adjustments
         let (size, align) = LinkedListAllocator::size_align(layout);
         let mut allocator = self.lock();
 
         if let Some((region, alloc_start)) = allocator.find_region(size, align) {
             let alloc_end = alloc_start.checked_add(size).expect("overflow");
             let excess_size = region.end_addr() - alloc_end;
             if excess_size > 0 {
                 allocator.add_free_region(alloc_end, excess_size);
             }
             alloc_start as *mut u8
         } else {
             core::ptr::null_mut()
         }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        // perform layout adjustments
        let (size, _) = LinkedListAllocator::size_align(layout);

        let mut allocator = self.lock();
        allocator.add_free_region(ptr as usize, size);
    }
}

