use crate::cell::MyCell;
use std::marker::PhantomData;
use std::ptr::NonNull;

struct RcInner<T> {
    value: T,
    refcount: MyCell<usize>,
}

pub struct MyRc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}

impl<T> MyRc<T> {
    pub fn new(value: T) -> Self {
        let inner = Box::new(RcInner {
            value,
            refcount: MyCell::new(0),
        });
        MyRc {
            // SAFETY: Box does not give us a null pointer
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        inner.refcount.set(c + 1);
        Self {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: self.inner is a Box that is only deallocated when the
        // last MyRc goes away
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        if c == 1 {
            // SAFETY: we are the only MyRc left, and we are being dropped
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            inner.refcount.set(c - 1);
        }
    }
}
