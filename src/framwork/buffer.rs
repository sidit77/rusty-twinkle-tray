use std::array::from_fn;
use std::cell::{Cell, UnsafeCell};
use std::mem::{MaybeUninit, needs_drop};
use std::ops::{Add, Rem};

pub struct CircularBuffer<T, const N: usize> {
    start: UnAtomicUsize,
    end: UnAtomicUsize,
    data: [UnsafeCell<MaybeUninit<T>>; N]
}

impl<T, const N: usize> Default for CircularBuffer<T, N> {
    fn default() -> Self {
        Self {
            start: UnAtomicUsize::new(0),
            end: UnAtomicUsize::new(0),
            data: from_fn(|_| UnsafeCell::new(MaybeUninit::uninit())),
        }
    }
}

impl<T, const N: usize> CircularBuffer<T, N> {
    pub fn try_push(&self, item: T) -> bool {
        let write_index = self
            .start
            .load();
        let next_write_index = write_index
            .add(1)
            .rem(self.data.len());
        (next_write_index != self.end.load())
            .then(|| {
                unsafe { self.data[write_index].get().write(MaybeUninit::new(item)) };
                self.start.store(next_write_index);
            })
            .is_some()
    }

    pub fn try_pop(&self) -> Option<T> {
        let read_index = self
            .end
            .load();
        (read_index != self.start.load())
            .then(|| {
                let data = unsafe { self.data[read_index].get().read().assume_init() };
                let next_read_index = read_index
                    .add(1)
                    .rem(self.data.len());
                self.end.store(next_read_index);
                data
            })
    }

}

impl<T, const N: usize> Drop for CircularBuffer<T, N> {
    fn drop(&mut self) {
        if needs_drop::<T>() {
            while let Some(val) = self.try_pop() {
                drop(val);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let buffer = CircularBuffer::<i32, 4>::default();

        assert_eq!(buffer.try_pop(), None);
        assert_eq!(buffer.try_push(3), true);
        assert_eq!(buffer.try_pop(), Some(3));
        assert_eq!(buffer.try_pop(), None);

    }
}

#[repr(transparent)]
struct UnAtomicUsize(Cell<usize>);

impl UnAtomicUsize {

    pub fn new(value: usize) -> Self {
        Self(Cell::new(value))
    }

    pub fn load(&self) -> usize {
        self.0.get()
    }

    pub fn store(&self, value: usize) {
        self.0.set(value)
    }

}