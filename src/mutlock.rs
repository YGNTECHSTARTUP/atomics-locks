use std::{sync::atomic::AtomicBool, thread};

pub fn hel() {
    println!("Hello world");
}

static LOCKED: AtomicBool = AtomicBool::new(false);
static mut DATA: String = String::new();
pub fn lc() {
    thread::scope(|s| {
        for _ in 0..1000000000 {
            s.spawn(|| f());
        }
    });

    unsafe { println!("FINAL DATA={}", DATA) }
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
        println!("BLOCK EXECUTED");
        unsafe {
            DATA.push('!');
        }
        LOCKED.store(false, std::sync::atomic::Ordering::Release);
    };
}
