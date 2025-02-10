use std::cell::Cell;
use std::marker::PhantomData;
use std::mem;
use std::ops::Deref;
use std::ptr::NonNull;

struct RcInner<T> {
    count: Cell<usize>,
    data: T,
}

struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}

impl<T> Rc<T> {
    fn new(data: T) -> Rc<T> {
        let inner = Box::new(RcInner {
            count: Cell::new(1),
            data,
        });

        Rc {
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }

    pub fn strong_count(&self) -> usize {
        // let inner = unsafe { self.inner.as_ref() };
        let inner = unsafe { &*self.inner.as_ptr() };
        inner.count.get()
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Rc<T> {
        let inner = unsafe { self.inner.as_ref() };
        inner.count.set(inner.count.get() + 1);

        Rc {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };

        if inner.count.get() > 1 {
            inner.count.set(inner.count.get() - 1);
        } else if inner.count.get() == 1 {
            mem::drop(inner);
            let _free = unsafe { Box::from_raw(self.inner.as_ptr()) };
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let inner = unsafe { self.inner.as_ref() };
        &inner.data
    }
}

#[test]
fn test_rc() {
    let a = Rc::new(123);
    let b = 456;

    let a_prime = Rc::clone(&a);
    println!("a + b = {}", b + *a_prime)
}
