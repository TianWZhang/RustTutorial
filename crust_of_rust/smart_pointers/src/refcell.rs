use crate::cell::MyCell;
use std::cell::UnsafeCell;

pub struct MyRefCell<T> {
    value: UnsafeCell<T>,
    reference: MyCell<isize>,
}

pub struct Ref<'refcell, T> {
    refcell: &'refcell MyRefCell<T>,
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.reference.get() {
            n if n <= 0 => unreachable!(),
            n => self.refcell.reference.set(n - 1),
        }
    }
}

impl<T> std::ops::Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: a Ref is only created if no exclusive reference has been given out
        unsafe { &*self.refcell.value.get() }
    }
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell MyRefCell<T>,
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.reference.get() {
            -1 => self.refcell.reference.set(0),
            _ => unreachable!(),
        }
    }
}

impl<T> std::ops::Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: a RefMut if only created if no other reference has been given out
        unsafe { &*self.refcell.value.get() }
    }
}

// The mutable reference that we return live only as long as the mutable reference to self
impl<T> std::ops::DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: a RefMut if only created if no other reference has been given out
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<T> MyRefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            reference: MyCell::new(0),
        }
    }

    // The return type cannot be Option<&T>, because we have no way to track when they go away.
    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        let reference = self.reference.get();
        if reference >= 0 {
            self.reference.set(reference + 1);
            Some(Ref { refcell: self })
        } else {
            None
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        let reference = self.reference.get();
        if reference == 0 {
            self.reference.set(-1);
            // SAFETY: no other reference has been given out
            Some(RefMut { refcell: self })
        } else {
            None
        }
    }
}
