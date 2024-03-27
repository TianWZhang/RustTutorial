use std::cell::UnsafeCell;

pub struct MyCell<T> {
    value: UnsafeCell<T>,
}

impl<T> MyCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY: We know no-one else is concurrently mutating self.value (because of !Sync).
        // We're not invalidating any references, because we never give any out.
        unsafe { *self.value.get() = value };
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: we know no-one else is mutating value, since only this thread can mutate
        // (because of !Sync), and it is executing this function instead.
        unsafe { *self.value.get() }
    }
}
