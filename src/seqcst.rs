use std::{fs::read_to_string, sync::atomic::AtomicBool, thread};

pub fn seq() {
    static A: AtomicBool = AtomicBool::new(false);
    static B: AtomicBool = AtomicBool::new(false);
    static mut C: String = String::new();
    thread::spawn(|| {
        A.store(true, std::sync::atomic::Ordering::SeqCst);
        println!("Data stored");
        if !B.load(std::sync::atomic::Ordering::SeqCst) {
            unsafe {
                C.push('!');
            }
        }
    })
    .join()
    .unwrap();

    thread::spawn(|| {
        B.store(true, std::sync::atomic::Ordering::SeqCst);

        println!("Data stored at B");
        if !A.load(std::sync::atomic::Ordering::SeqCst) {
            unsafe {
                C.push('!');
            }
        }
    })
    .join()
    .unwrap();
    unsafe {
        println!("{:?}", C.len());
    }
}

