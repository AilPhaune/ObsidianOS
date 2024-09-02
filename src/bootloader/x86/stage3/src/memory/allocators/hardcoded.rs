use core::marker::PhantomData;

use crate::memory::{AllocatedMemory, MemoryAllocator};

pub struct HardcodedAddressAllocator<'a> {
    begin: usize,
    size: usize,
    allocated: bool,
    dropped: bool,
    phantom: PhantomData<&'a ()>,
}

impl<'a> HardcodedAddressAllocator<'a> {
    pub fn new(begin: usize, size: usize) -> Self {
        Self {
            begin,
            size,
            allocated: false,
            dropped: false,
            phantom: PhantomData,
        }
    }

    fn get_ptr<T>(&mut self) -> *mut T {
        self.begin as *mut T
    }

    pub fn deallocate(mut self) {
        self.dropped = true;
    }
}

impl<'a> MemoryAllocator<'a> for HardcodedAddressAllocator<'a> {
    fn size(&mut self) -> usize {
        self.size
    }

    fn remaining_free(&mut self) -> usize {
        if self.allocated {
            0
        } else {
            self.size()
        }
    }

    fn allocate<'b: 'a>(&mut self, size_bytes: usize) -> Option<AllocatedMemory<'b, ()>> {
        if self.allocated || self.size() < size_bytes {
            None
        } else {
            Some(AllocatedMemory::<()>::new(
                self.get_ptr::<()>(),
                self.size(),
            ))
        }
    }

    fn free<'b: 'a>(&mut self, ptr: AllocatedMemory<'b, ()>) -> bool {
        if self.allocated || self.dropped || ptr.pointer != self.get_ptr::<()>() {
            false
        } else {
            self.allocated = false;
            true
        }
    }
}
