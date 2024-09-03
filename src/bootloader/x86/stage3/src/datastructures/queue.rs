use core::marker::PhantomData;

pub struct Queue<'a, T: Sized + Clone> {
    ptr: *mut T,
    /// In `T`
    capacity: usize,
    /// In `T`
    index: usize,
    /// In `T`
    length: usize,

    phantom: PhantomData<&'a ()>,
}

impl<'a, T: Sized + Clone> Queue<'a, T> {
    pub fn new(ptr: *mut T, capacity: usize) -> Self {
        Self {
            ptr,
            capacity,
            index: 0,
            length: 0,
            phantom: PhantomData,
        }
    }

    /// Returns the first element of the queue, but does not remove it
    pub fn get(&self) -> Option<&'a T> {
        if self.length == 0 {
            None
        } else {
            Some(unsafe { &*(self.ptr.add(self.index)) })
        }
    }

    /// Returns the first element of the queue as mutable, but does not remove it
    pub fn get_mut(&mut self) -> Option<&'a mut T> {
        if self.length == 0 {
            None
        } else {
            Some(unsafe { &mut *(self.ptr.add(self.index)) })
        }
    }

    /// Removes and return the first element of the queue
    pub fn dequeue(&mut self) -> Option<T> {
        if self.length == 0 {
            None
        } else {
            Some(unsafe {
                let value = (*(self.ptr.add(self.index))).clone();
                self.index += 1;
                if self.index >= self.capacity {
                    self.index = 0;
                }
                self.length -= 1;
                value
            })
        }
    }

    pub fn add(&mut self, value: T) {
        if self.capacity == 0 {
            return;
        }
        let idx = (self.index + self.length) % self.capacity;
        unsafe {
            *(self.ptr.add(idx)) = value;
        }
        self.length += 1;
    }
}
