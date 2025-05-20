use std::cell::UnsafeCell;

pub fn un() {
    struct MyCell<T> {
        value: UnsafeCell<T>,
    }
    impl<T> MyCell<T> {
        pub fn new(val: T) -> MyCell<T> {
            MyCell {
                value: UnsafeCell::new(val),
            }
        }
        pub fn get(&self) -> T
        where
            T: Copy,
        {
            unsafe { *self.value.get() }
        }
        pub fn set(&mut self, val: T) {
            unsafe { *self.value.get() = val }
        }
    }

    let mut cell = MyCell::new(10);
    let k = cell.get();
    println!("{:?}", k);
    cell.set(20);
    let k = cell.get();
    println!("{:?}", k)
}

pub fn un1() {
    struct MyCell<T> {
        value: T,
    }
    impl<T> MyCell<T> {
        pub fn new(val: T) -> MyCell<T> {
            MyCell { value: val }
        }
        pub fn get(&self) -> &T {
            &self.value
        }
        pub fn set(&mut self, val: T) {
            self.value = val
        }
    }

    let mut cell = MyCell::new(10);
    let k = cell.get();
    println!("{:?}", k);
    cell.set(20);
    let k = cell.get();
    println!("{:?}", k)
}
