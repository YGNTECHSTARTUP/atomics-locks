use std::{
    sync::{
        atomic::{AtomicBool, AtomicI32, AtomicU32},
        Arc, Condvar, Mutex,
    },
    thread,
    time::Duration,
};

use crossbeam::atomic::AtomicCell;

pub fn cv() {
    let a = Arc::new((Mutex::new(false), Condvar::new()));
    thread::scope(|s| {
        for _ in 0..100 {
            let a = Arc::clone(&a);
            s.spawn(move || {
                let (lc, cvc) = &*a;
                let mut lc = lc.lock().unwrap();
                *lc = !*lc;
                if *lc == true {
                    println!("Thread Notified");
                    cvc.notify_one();
                }
            });
        }
        for _ in 0..100 {
            let a = Arc::clone(&a);
            s.spawn(move || {
                let (lc, cvc) = &*a;
                let mut lc = lc.lock().unwrap();
                *lc = !*lc;
                if *lc == false {
                    println!("Thread Waited");
                    let cvc = cvc.wait(lc).unwrap();
                }
            });
        }
    })
}

pub fn mm() {
    let a: AtomicBool = AtomicBool::new(false);
    let b: AtomicU32 = AtomicU32::new(10);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| b.fetch_add(10, std::sync::atomic::Ordering::Relaxed));
        }
        for _ in 0..10 {
            s.spawn(|| {
                let a = b.load(std::sync::atomic::Ordering::Relaxed);
                println!("{:?}", a);
            });
        }
    });

    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| b.fetch_add(10, std::sync::atomic::Ordering::Release));
        }
        for _ in 0..10 {
            s.spawn(|| {
                let a = b.load(std::sync::atomic::Ordering::Acquire);
                println!("{:?}", a);
            });
        }
    });
}
