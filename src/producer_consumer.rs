
use std::{
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
    thread::{self, spawn},
    time::Duration,
};
  const FULL_CAPACITY: usize = 10;
  pub fn abc(){
          let v = Arc::new((Mutex::new(VecDeque::new()), Condvar::new(), Condvar::new()));

    for _ in 0..10 {
        let v = Arc::clone(&v);
        spawn(move || {
            let (q, isfull, isempty) = &*v;
            let mut q = q.lock().unwrap();
            while q.is_empty() {
                q = isempty.wait(q).unwrap();
            }
            let vv = q.pop_front();
            println!("Got {:?}", vv);
            isfull.notify_one();
        });
    }
    for i in 0..10 {
        let v = Arc::clone(&v);
        spawn(move || {
            let (q, isfull, isempty) = &*v;
            let mut q = q.lock().unwrap();
            while q.len() == FULL_CAPACITY {
                q = isfull.wait(q).unwrap();
            }
            let vv = q.push_front(i);
            println!("pushed {:?}", vv);
            isempty.notify_one();
            thread::sleep(Duration::from_millis(300));
        });
    }

    thread::sleep(Duration::from_secs(300));
  }
