use core::marker::PhantomData;

pub struct Vector<'a, T: Sized> {
    ptr: *mut T,
    length: usize,
    capacity: usize,

    phantom: PhantomData<&'a ()>,
}

impl<'a, T: Sized + Clone> Vector<'a, T> {
    pub fn new(ptr: *mut T, capacity: usize) -> Self {
        Self {
            ptr,
            length: 0,
            capacity,
            phantom: PhantomData,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn push(&mut self, value: T) -> bool {
        if self.length >= self.capacity {
            false
        } else {
            unsafe {
                *(self.ptr.add(self.length)) = value;
            }
            self.length += 1;
            true
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            None
        } else {
            self.length -= 1;
            Some(unsafe { (*(self.ptr.add(self.length))).clone() })
        }
    }

    pub fn get(&self, index: usize) -> Option<&'a T> {
        if index >= self.length {
            None
        } else {
            Some(unsafe { &*(self.ptr.add(index)) })
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&'a mut T> {
        if index >= self.length {
            None
        } else {
            Some(unsafe { &mut *(self.ptr.add(index)) })
        }
    }
}
