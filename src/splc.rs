use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::{atomic::AtomicBool, Arc},
    thread,
};

struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

struct MutexGuarde<'a, T> {
    guard: &'a SpinLock<T>,
}

impl<T> SpinLock<T> {
    pub fn new(n: T) -> SpinLock<T> {
        SpinLock {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(n),
        }
    }
    pub fn lock(&self) -> MutexGuarde<T> {
        if self.locked.swap(true, std::sync::atomic::Ordering::Acquire) {
            println!("Waiting in the Lock");
            std::hint::spin_loop();
        }
        MutexGuarde { guard: &self }
    }
}
unsafe impl<T> Sync for SpinLock<T> where T: Send {}

unsafe impl<T> Sync for MutexGuarde<'_, T> where T: Sync {}
unsafe impl<T> Send for MutexGuarde<'_, T> where T: Send {}

impl<T> Drop for MutexGuarde<'_, T> {
    fn drop(&mut self) {
        self.guard
            .locked
            .store(false, std::sync::atomic::Ordering::Release);
    }
}

impl<T> DerefMut for MutexGuarde<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.guard.value.get() }
    }
}

impl<T> Deref for MutexGuarde<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.guard.value.get() }
    }
}

pub fn spl() {
    let a = Arc::new(SpinLock::new(10));
    thread::scope(|s| {
        for i in 0..10 {
            let a = Arc::clone(&a);
            s.spawn(move || {
                let mut a = a.lock();
                println!("{:?}", *a);
                *a += i;
            });
        }
        for i in 0..10 {
            println!("{i}th time {:?}", *a.lock());
        }
    })
}
