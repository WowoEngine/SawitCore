use virtio_drivers::{Hal, BufferDirection};
use crate::memory::{FRAME_ALLOCATOR, PHYSICAL_MEMORY_OFFSET};
use x86_64::{
    structures::paging::FrameAllocator,
    VirtAddr as X64VirtAddr,
};
use core::ptr::NonNull;

pub struct VirtioHal;

unsafe impl Hal for VirtioHal {
    fn dma_alloc(pages: usize, _direction: BufferDirection) -> (usize, NonNull<u8>) {
        let mut allocator = FRAME_ALLOCATOR.lock();
        if let Some(allocator) = allocator.as_mut() {
            let start_frame = allocator.allocate_frame().expect("DMA Allocation failed: No frames");
            let start_addr = start_frame.start_address().as_u64();
            
            // Verify contiguity (heuristic)
            let mut current_addr = start_addr + 4096;
            for _ in 1..pages {
                 let frame = allocator.allocate_frame().expect("DMA Allocation failed: OOM");
                 if frame.start_address().as_u64() != current_addr {
                     panic!("DMA Allocation failed: Fragmentation");
                 }
                 current_addr += 4096;
            }
            
            let start_virt = unsafe { start_addr + PHYSICAL_MEMORY_OFFSET };
            let ptr = start_virt as *mut u8;
            unsafe { ptr.write_bytes(0, pages * 4096); }

            // Ensure NonNull
            return (start_addr as usize, NonNull::new(ptr).unwrap());
        }
        panic!("DMA Initialized too early");
    }

    unsafe fn dma_dealloc(_paddr: usize, _vaddr: NonNull<u8>, _pages: usize) -> i32 {
        0
    }

    unsafe fn share(buffer: NonNull<[u8]>, _direction: BufferDirection) -> usize {
        // x86_64 is coherent, no-op; return physical address
        let vaddr = buffer.as_ptr() as *mut u8 as usize;
        unsafe { vaddr - PHYSICAL_MEMORY_OFFSET as usize }
    }

    unsafe fn unshare(_phys: usize, _buffer: NonNull<[u8]>, _direction: BufferDirection) {
        // no-op
    }

    unsafe fn mmio_phys_to_virt(paddr: usize, _size: usize) -> NonNull<u8> {
        let vaddr = unsafe { paddr + PHYSICAL_MEMORY_OFFSET as usize };
        NonNull::new(vaddr as *mut u8).unwrap()
    }
}
