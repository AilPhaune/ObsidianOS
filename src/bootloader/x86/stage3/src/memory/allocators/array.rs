use crate::memory::{AllocatedMemory, MemoryAllocator};

pub struct ArrayAllocator<'a> {
    array: &'a mut [u8],
    allocated: bool,
    dropped: bool,
}

impl<'a> ArrayAllocator<'a> {
    pub fn new(array: &'a mut [u8]) -> Self {
        Self {
            array,
            allocated: false,
            dropped: false,
        }
    }

    fn get_ptr<T>(&mut self) -> *mut T {
        unsafe { self.array.get_unchecked_mut(0) as *mut u8 as *mut T }
    }
}

impl<'a> MemoryAllocator<'a> for ArrayAllocator<'a> {
    fn size(&mut self) -> usize {
        self.array.len()
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
