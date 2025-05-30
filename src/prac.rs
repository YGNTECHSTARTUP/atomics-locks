use std::{
    collections::VecDeque,
    ptr::null_mut,
    sync::{
        atomic::{fence, AtomicBool, AtomicPtr, AtomicU32, AtomicU64, AtomicUsize},
        Arc, Condvar, Mutex, OnceLock,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

pub fn aaa() {
    let a: Arc<(Mutex<VecDeque<u32>>, Condvar)> =
        Arc::new((Mutex::new(VecDeque::new()), Condvar::new()));
    //producer
    for i in 0..10 {
        let a = Arc::clone(&a);
        thread::spawn(move || {
            let (lock, cvar) = &*a;
            let mut queue = lock.lock().unwrap();
            queue.push_front(i);
            cvar.notify_one();
        });
    }

    for _ in 0..10 {
        let a = Arc::clone(&a);
        thread::spawn(move || {
            let (lock, cvar) = &*a;
            let mut queue = lock.lock().unwrap();
            while queue.is_empty() {
                queue = cvar.wait(queue).unwrap();
            }
            if let Some(item) = queue.pop_front() {
                println!("Got: {:?}", item);
            }
        });
    }
    thread::sleep(Duration::from_secs(10));
}

pub fn dd() {
    let num_done = AtomicUsize::new(0);
    let time_takene = AtomicUsize::new(0);
    let peak_time = AtomicUsize::new(0);
    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(|| {
                let start = Instant::now();
                comp(4);
                let time_taken = start.elapsed().as_millis() as usize;
                num_done.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                time_takene.fetch_add(time_taken, std::sync::atomic::Ordering::Relaxed);
                peak_time.fetch_max(time_taken, std::sync::atomic::Ordering::Relaxed);
            });
        }
        loop {
            let n = num_done.load(std::sync::atomic::Ordering::Relaxed);
            if n == 100 {
                return;
            }
            if n == 0 {
                println!("Starting to Perform the Operation");
            } else {
                let total_time = time_takene.load(std::sync::atomic::Ordering::Relaxed);
                let peak_time = peak_time.load(std::sync::atomic::Ordering::Relaxed);
                println!(
                    "Total_Time {:?}ms  -->   Peak_Time {:?}ms    -->   Tasks{:?}",
                    total_time, peak_time, n
                );
            }
        }
    })
}

pub fn comp(x: usize) {
    thread::sleep(Duration::from_millis((x * 31) as u64));
}

pub fn ac() {
    static NEXT_ID: AtomicU64 = AtomicU64::new(1);
    const LIMIT: u64 = 1000;
    loop {
        let id = NEXT_ID.load(std::sync::atomic::Ordering::Relaxed);
        if id >= LIMIT {
            return;
        } else {
            let c = NEXT_ID
                .compare_exchange(
                    id,
                    id + 1,
                    std::sync::atomic::Ordering::Relaxed,
                    std::sync::atomic::Ordering::Relaxed,
                )
                .unwrap();
            let c = format!("Thread {:?}", c);
            let th = thread::Builder::new().name(c.into());
            println!("{:?}", th);
        }
    }
}

pub fn aw() {
    loop {
        const LIMIT: u32 = 1000;
        static NEXT_ID: AtomicU32 = AtomicU32::new(0);
        let k = NEXT_ID
            .fetch_update(
                std::sync::atomic::Ordering::Relaxed,
                std::sync::atomic::Ordering::Relaxed,
                |n| {
                    if n < LIMIT {
                        Some(n + 1)
                    } else {
                        None
                    }
                },
            )
            .ok()
            .expect("WRONG");
        let k = format!("Thread {:?}", k);
        let th = thread::Builder::new().name(k.into());
        println!("{:?}", th);
    }
}

pub fn ou() {
    static LOCKED: AtomicBool = AtomicBool::new(false);
    static mut DATA: u32 = 10;
    for i in 0..100 {
        thread::spawn(move || f());
    }
    unsafe {
        println!("{:?}", DATA);
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
                DATA += 10;
            }
        }
        LOCKED.store(false, std::sync::atomic::Ordering::Release);
    }
}

pub fn ptr() {
    let a = Arc::new(AtomicU32::new(10));
    for i in 0..100 {
        let a = Arc::clone(&a);
        thread::spawn(move || {
            let current = a.load(std::sync::atomic::Ordering::Acquire);
            if a.compare_exchange(
                current,
                current + 1,
                std::sync::atomic::Ordering::Acquire,
                std::sync::atomic::Ordering::Relaxed,
            )
            .is_ok()
            {
                println!("Updated {current}")
            }
        });
    }

    for i in 0..100 {
        let a = Arc::clone(&a);
        thread::spawn(move || {
            let current = a.load(std::sync::atomic::Ordering::Acquire);
            println!("Current:{:?}", current)
        });
    }
    thread::sleep(Duration::from_secs(1));
    println!("{:?}", a.load(std::sync::atomic::Ordering::Acquire))
}

pub fn lev() {
    let a: Arc<AtomicU32> = Arc::new(AtomicU32::new(10));
    let mut handles: Vec<JoinHandle<()>> = vec![];
    for i in 0..100 {
        let a = Arc::clone(&a);
        let handler = thread::spawn(move || {
            a.fetch_add(20, std::sync::atomic::Ordering::Release);
        });
        handles.push(handler);
    }

    for i in 0..100 {
        let a = Arc::clone(&a);
        let handler = thread::spawn(move || {
            a.fetch_add(120, std::sync::atomic::Ordering::Acquire);
        });
        handles.push(handler);
    }
    for hand in handles {
        hand.join().unwrap()
    }

    let b = a.load(std::sync::atomic::Ordering::Acquire);
    println!("{:?}", b);
}

// pub fn az(){
//     static PTR:AtomicPtr<i32> = AtomicPtr::new(null_mut());
//     let p = PTR.load(std::sync::atomic::Ordering::Acquire);
//     if p.is_null(){
//         let data = Box::into_raw(Box::new(3));
//       if let Err(e)= PTR.compare_exchange(null_mut(),data,std::sync::atomic::Ordering::Release,std::sync::atomic::Ordering::Acquire){
//       drop(unsafe {
//           Box::from_raw(data);
//     });
//     println!("hh");
//   };
//     }
// }

pub fn sp() {
    let a = AtomicBool::new(false);
}

pub fn ak() {
    let a = AtomicU32::new(10);

    a.fetch_add(10, std::sync::atomic::Ordering::Release);
    thread::spawn(move || {
        let aa = a.load(std::sync::atomic::Ordering::Acquire);
        println!("{:?}", aa);
    })
    .join()
    .unwrap();
}

pub fn aww() {
    static A: AtomicU32 = AtomicU32::new(0);

    static B: AtomicU32 = AtomicU32::new(0);
    static C: AtomicU32 = AtomicU32::new(0);
    static D: AtomicU32 = AtomicU32::new(0);
    thread::spawn(|| {
        fence(std::sync::atomic::Ordering::Release);
        A.store(10, std::sync::atomic::Ordering::Relaxed);
        B.store(20, std::sync::atomic::Ordering::Relaxed);
        C.store(30, std::sync::atomic::Ordering::Relaxed);
        D.store(40, std::sync::atomic::Ordering::Relaxed);
    });

    thread::spawn(|| {
        A.load(std::sync::atomic::Ordering::Relaxed);
        B.load(std::sync::atomic::Ordering::Relaxed);
        C.load(std::sync::atomic::Ordering::Relaxed);
        D.load(std::sync::atomic::Ordering::Relaxed);
        fence(std::sync::atomic::Ordering::Acquire);
        println!("{:?}{:?}{:?}{:?}", A, B, C, D);
    });
}
