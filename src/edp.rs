use std::{
    sync::{atomic::AtomicUsize, Arc},
    thread,
    time::{Duration, Instant},
};

pub fn a() {
    let a = thread::Builder::new()
        .name("a".into())
        .stack_size(50 * 1024);
    println!("{:?}", a);
}
pub fn b() {
    let a = Arc::new(10);
    println!("{:?}", a);
}

pub fn ee() {
    fn comp(id: usize) {
        println!("Computing {id}...");
        thread::sleep(Duration::from_millis(20 + (id % 10) as u64));
    }

    let num = &AtomicUsize::new(0);
    let total_time = &AtomicUsize::new(0);
    let peak = &AtomicUsize::new(0);
    thread::scope(|s| {
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    let start = Instant::now();
                    comp(t * 25 + i + 1);
                    let time_taken = start.elapsed().as_micros() as usize;
                    total_time.fetch_add(time_taken, std::sync::atomic::Ordering::Relaxed);
                    num.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    peak.fetch_max(time_taken, std::sync::atomic::Ordering::Relaxed);
                }
            });
        }
        loop {
            let n = num.load(std::sync::atomic::Ordering::Relaxed);
            if n == 100 {
                break;
            }

            if n == 0 {
                println!("Waiting for the task!");
            } else {
                let total_time = total_time.load(std::sync::atomic::Ordering::Relaxed);
                let peak_time = peak.load(std::sync::atomic::Ordering::Relaxed);
                let avg = Duration::from_micros((total_time / n) as u64);
                let peak = Duration::from_micros(peak_time as u64);
                println!("Working on {n} avg:{:?} peaktime is {:?}", avg, peak);
            }
            thread::sleep(Duration::from_millis(50));
        }
    });
    println!("Operation DOne!");
}
