use std::{
    cell::{Ref, UnsafeCell},
    ops::Deref,
    process,
    ptr::NonNull,
    sync::atomic::{fence, AtomicUsize},
    thread,
    time::Duration,
};

struct ArcData<T> {
    ref_count: AtomicUsize,
    data: T,
}

struct Arc<T> {
    ptr: NonNull<ArcData<T>>,
}

impl<T> Arc<T> {
    pub fn new(n: T) -> Arc<T> {
        Arc {
            ptr: NonNull::from(Box::leak(Box::new(ArcData {
                ref_count: AtomicUsize::new(1),
                data: n,
            }))),
        }
    }
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }
    pub fn clone(&self) -> Self {
        if self
            .data()
            .ref_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            > usize::MAX / 2
        {
            process::abort()
        }
        Self { ptr: self.ptr }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self
            .data()
            .ref_count
            .fetch_sub(1, std::sync::atomic::Ordering::Release)
            == 1
        {
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

pub fn a() {
    let a = vec![19, 2, 21, 3, 1];
    for i in 0..10 {
        let b = a.clone();
        thread::spawn(move || {
            println!("{:?}", b);
        });
    }
    thread::sleep(Duration::from_secs(5));
}

pub struct WeakArc<T> {
    weak: Weak<T>,
}

pub struct Weak<T> {
    ptr: NonNull<WeakArcData<T>>,
}
pub struct WeakArcData<T> {
    ref_count: AtomicUsize,
    alloc_count: AtomicUsize,
    data: UnsafeCell<Option<T>>,
}

impl<T> Weak<T> {
    fn data(&self) -> &WeakArcData<T> {
        unsafe { self.ptr.as_ref() }
    }
    pub fn upgrade(&self) -> Option<Arc<T>> {
        let mut n = self.data().ref_count.load(Relaxed);
        loop {
            if n ==0 {
                return None;
            }
        
        if let Err(e) = self.data().ref_count.compare_exchange_weak(n,n+1,std::sync::atomic::Ordering::Relaxed ,std::sync::atomic::Ordering::Relaxed ){
            n = e;
            continue;
        }
        return Some(WeakArc {
            weak:self.clone()
        })
         }
    
}

impl<T> Deref for WeakArc<T> {
    type Target = T;
    fn deref(&self) -> &T {
        let ptr = self.weak.data().data.get();
        unsafe { (*ptr).as_ref().unwrap() }
    }
}

impl<T> WeakArc<T> {
    pub fn new(data: T) -> WeakArc<T> {
        WeakArc {
            weak: Weak {
                ptr: NonNull::from(Box::leak(Box::new(WeakArcData {
                    ref_count: AtomicUsize::new(1),
                    alloc_count: AtomicUsize::new(1),
                    data: UnsafeCell::new(Some(data)),
                }))),
            },
        }
    }
    pub fn get_mut(arc:&mut Self) -> Option<&mut T> {
        if arc.weak.data().alloc_count.load(std::sync::atomic::Ordering::Relaxed) == 1 {
            fence(std::sync::atomic::Ordering::Acquire);
            let arcdata = unsafe {
                arc.weak.ptr.as_mut()
            };
            let option = arcdata.data.get_mut();
            let data = option.as_mut().unwrap();
            Some(data)
        }
        else {
            None
        }
    }
    pub fn downgrade(a:&Self) -> Weak<T> {
        a.weak.clone()
    }
}

impl<T> Clone for Weak<T> {
    fn clone(&self) -> Self {
        if self
            .data()
            .alloc_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            > usize::MAX / 2
        {
            std::process::abort()
        }
        Weak { ptr: self.ptr }
    }
}

impl<T> Clone for WeakArc<T> {
    fn clone(&self) -> Self {
        let weak = self.weak.clone();
        if weak
            .data()
            .ref_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            > usize::MAX / 2
        {
            std::process::abort();
        }
        WeakArc { weak }
    }
}

impl<T> Drop for Weak<T> {
    fn drop(&mut self) {
        if self
            .data()
            .alloc_count
            .fetch_sub(1, std::sync::atomic::Ordering::Release)
            == 1
        {
            fence(std::sync::atomic::Ordering::Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()))
            }
        }
    }
}

impl <T> Drop for WeakArc<T> {
    fn drop(&mut self) {
        if self.weak.data().ref_count.fetch_sub(1,std::sync::atomic::Ordering::Release) == 1 {
         fence(std::sync::atomic::Ordering::Acquire);
         let ptr = self.weak.data().data.get();
         unsafe {
             (*ptr) = None;
         }   
        }
    }
}




unsafe impl<T> Sync for Weak<T> where T: Send + Sync {}
unsafe impl<T> Send for Weak<T> where T: Send + Sync {}
