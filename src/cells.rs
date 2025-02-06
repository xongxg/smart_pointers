use std::cell::UnsafeCell;
use std::ops::Deref;

pub struct MyCell<T> {
    value: UnsafeCell<T>,
}

impl<T> MyCell<T> {
    pub fn new(value: T) -> Self {
        MyCell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }

    pub fn set(&self, value: T) {
        unsafe {
            *self.value.get() = value;
        }
    }
}

// impl<T> !Sync for MyCell<T> {}

impl<T> Deref for MyCell<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.value.get() }
    }
}

#[cfg(test)]
mod tests {
    use crate::cells::MyCell;

    #[test]
    fn test_my_cell() {
        let cell = MyCell::new(10);
        assert_eq!(cell.get(), 10);

        cell.set(20);
        assert_eq!(cell.get(), 20);
    }
}
