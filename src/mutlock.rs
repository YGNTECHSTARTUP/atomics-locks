use std::{sync::atomic::AtomicBool, thread};

pub fn hel() {
    println!("Hello world");
}

static LOCKED: AtomicBool = AtomicBool::new(false);
static mut DATA: String = String::new();
pub fn lc() {
    for id in 0..100 {
        thread::spawn(|| f());
    }
}

fn f() {
    if LOCKED
        .compare_exchange(
            false,
            true,
            std::sync::atomic::Ordering::Acquire,
            std::sync::atomic::Ordering::Relaxed,
        )
        .is_ok()
    {
        unsafe {
            DATA.push('!');
        }
        println!("BLOCK EXECUTED");
        LOCKED.store(false, std::sync::atomic::Ordering::Release);
    };
}
