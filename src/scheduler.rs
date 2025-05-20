use std::{
    cell::{Cell, UnsafeCell},
    sync::{Arc, Mutex},
    thread::{self, current, JoinHandle},
};

pub fn abb() {
    let mut handles: Vec<(JoinHandle<()>, thread::Thread)> = vec![];
    let x = Arc::new(Mutex::new(10));
    for _ in 0..10 {
        let x = Arc::clone(&x);
        let handle = thread::spawn(move || {
            let current = thread::current();
            let mut x = x.lock().unwrap();
            *x += 10;
            if *x > 50 {
                println!("Parking the thread with id {:?}", current.id());
            }
        });
        let thread_ref = handle.thread().clone();
        handles.push((handle, thread_ref));
    }
    for (_, t) in &handles {
        println!("Unparking the thread with id {:?}", t.id());
        t.unpark();
    }
    for (h, _) in handles {
        h.join().unwrap();
    }
}

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
