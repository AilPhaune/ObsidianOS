use core::marker::PhantomData;

use crate::memory::{AllocatedMemory, MemoryAllocator};

pub struct ArenaAllocator<'a> {
    allocator: &'a mut dyn MemoryAllocator<'a>,
    memory: AllocatedMemory<'a, u8>,
    index: usize,
    dropped: bool,
}

unsafe impl<'a> Sync for ArenaAllocator<'a> {}

impl<'a> ArenaAllocator<'a> {
    pub fn new<T>(allocator: &'a mut T, size: usize) -> Option<Self>
    where
        T: MemoryAllocator<'a>,
    {
        allocator.allocate(size).map(|v| Self {
            memory: v.with_type::<u8>(),
            index: 0,
            dropped: false,
            allocator,
        })
    }
}

impl<'a> ArenaAllocator<'a> {
    pub fn deallocate(&mut self) {
        self.allocator.free(self.memory.with_type());
        self.dropped = true;
    }
}

impl<'a> MemoryAllocator<'a> for ArenaAllocator<'a> {
    fn size(&mut self) -> usize {
        self.memory.size
    }
    fn remaining_free(&mut self) -> usize {
        self.memory.size - self.index
    }
    fn allocate<'b: 'a>(&mut self, size_bytes: usize) -> Option<AllocatedMemory<'b, ()>> {
        if size_bytes > self.remaining_free() {
            None
        } else {
            let prev_index = self.index;
            self.index += size_bytes;
            unsafe {
                Some(AllocatedMemory::<'b> {
                    pointer: self.memory.pointer.add(prev_index) as _,
                    size: size_bytes,
                    _lifetime: PhantomData,
                })
            }
        }
    }
    fn free<'b: 'a>(&mut self, _: AllocatedMemory<'b, ()>) -> bool {
        true
    }
}
