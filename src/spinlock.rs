use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::{atomic::AtomicBool, Arc},
    thread::{self, JoinHandle},
};

unsafe impl<T> Sync for Guard<'_, T> where T: Sync {}
unsafe impl<T> Send for Guard<'_, T> where T: Send {}
unsafe impl<T> Sync for SpinLock<T> where T: Send {}
struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock
            .locked
            .store(false, std::sync::atomic::Ordering::Release);
    }
}

impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> SpinLock<T> {
    pub fn new(val: T) -> SpinLock<T> {
        SpinLock {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(val),
        }
    }
    pub fn lock<'a>(&'a self) -> Guard<'a, T> {
        if self.locked.swap(true, std::sync::atomic::Ordering::Acquire) {
            println!("Waiting!");
            std::hint::spin_loop()
        }
        Guard { lock: &self }
    }
    pub fn unlock(&self) {
        self.locked
            .store(true, std::sync::atomic::Ordering::Release);
    }
}

pub fn sp() {
    let x = SpinLock::new(Vec::new());
    thread::scope(|s| {
        s.spawn(|| {
            x.lock().push(1);
        });
        s.spawn(|| {
            let mut g = x.lock();
            g.push(12);
            g.push(23);
        });
    });
    let g = x.lock();
    println!("{:?}", *g);
}

// pub fn sp() {
//     let a = Arc::new(SpinLock::new(10));
//     let mut handles: Vec<JoinHandle<()>> = vec![];
//     for _ in 0..100 {
//         let a = Arc::clone(&a);
//         let handle = thread::spawn(move || {
//             let a = a.lock();
//             println!("{:?}", a);
//         });
//         handles.push(handle);
//     }

//     for _ in 0..100 {
//         let a = Arc::clone(&a);
//         let handle = thread::spawn(move || {
//             a.unlock();
//             println!("Unlocked");
//         });
//         handles.push(handle);
//     }
//     for h in handles {
//         h.join().unwrap();
//     }
//     println!("{:?}", a.locked.load(std::sync::atomic::Ordering::Acquire))
// }
// }
