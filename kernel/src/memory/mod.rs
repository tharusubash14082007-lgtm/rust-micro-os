
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::sync::atomic::{AtomicUsize, Ordering};

const HEAP_SIZE: usize = 64 * 1024;

static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

pub struct BumpAllocator {
    next: AtomicUsize,
}

impl BumpAllocator {
    pub const fn new() -> Self {
        Self {
            next: AtomicUsize::new(0),
        }
    }
}

unsafe impl Sync for BumpAllocator {}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let heap_start = core::ptr::addr_of_mut!(HEAP) as *mut u8 as usize;

        let mut current = self.next.load(Ordering::Relaxed);

        loop {
            let aligned = match (heap_start + current)
                .checked_add(layout.align() - 1)
            {
                Some(value) => value & !(layout.align() - 1),
                None => return null_mut(),
            };

            let new_next = match aligned
                .checked_add(layout.size())
            {
                Some(value) => value - heap_start,
                None => return null_mut(),
            };

            if new_next > HEAP_SIZE {
                return null_mut();
            }

            match self.next.compare_exchange(
                current,
                new_next,
                Ordering::SeqCst,
                Ordering::Relaxed,
            ) {
                Ok(_) => return aligned as *mut u8,
                Err(actual) => current = actual,
            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator does not reclaim memory.
    }
}

#[global_allocator]
pub static ALLOCATOR: BumpAllocator = BumpAllocator::new();

pub fn test_allocation() -> bool {
    let layout = Layout::from_size_align(128, 8).unwrap();

    let ptr = unsafe {
        ALLOCATOR.alloc(layout)
    };

    if ptr.is_null() {
        return false;
    }

    unsafe {
        ptr.write_bytes(0xAA, 128);
    }

    true
}
