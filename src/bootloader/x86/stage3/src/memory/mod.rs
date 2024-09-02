use core::marker::PhantomData;

pub mod allocators;

pub struct AllocatedMemory<'a, T> {
    pointer: *mut T,
    size: usize,

    _lifetime: PhantomData<&'a ()>,
}

impl<'a, T> AllocatedMemory<'a, T> {
    pub fn new(pointer: *mut T, size: usize) -> Self {
        Self {
            pointer,
            size,
            _lifetime: PhantomData,
        }
    }

    pub fn get(&self) -> *mut T {
        self.pointer
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn with_type<'b: 'a, U>(&self) -> AllocatedMemory<'b, U> {
        AllocatedMemory {
            pointer: self.pointer as *mut U,
            size: self.size,

            _lifetime: PhantomData,
        }
    }
}

#[allow(drop_bounds)]
pub trait MemoryAllocator<'a> {
    fn size(&mut self) -> usize;
    fn remaining_free(&mut self) -> usize;
    fn allocate<'b: 'a>(&mut self, size_bytes: usize) -> Option<AllocatedMemory<'b, ()>>;
    fn free<'b: 'a>(&mut self, ptr: AllocatedMemory<'b, ()>) -> bool;
}
