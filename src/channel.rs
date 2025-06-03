// use core::panic;
// use std::{
//     cell::UnsafeCell,
//     collections::VecDeque,
//     mem::MaybeUninit,
//     sync::{
//         atomic::{AtomicBool, AtomicU8},
//         mpsc::channel,
//         Arc, Condvar, Mutex,
//     },
//     thread::{self, JoinHandle},
// };

// struct Channel<T> {
//     queue: Mutex<VecDeque<T>>,
//     c: Condvar,
// }
// impl<T> Channel<T> {
//     pub fn new() -> Self {
//         Channel {
//             queue: Mutex::new(VecDeque::new()),
//             c: Condvar::new(),
//         }
//     }
//     pub fn send(&self, message: T) {
//         self.queue.lock().unwrap().push_back(message);
//         self.c.notify_one();
//     }
//     pub fn receive(&self) -> T {
//         let mut b = self.queue.lock().unwrap();
//         loop {
//             if let Some(message) = b.pop_front() {
//                 return message;
//             }
//             b = self.c.wait(b).unwrap();
//         }
//     }
// }

// pub fn cn() {
//     let a = Arc::new(Channel::new());
//     a.send(10);
//     let b = a.receive();
//     println!("{:?}", b);
//     let mut handles: Vec<JoinHandle<()>> = vec![];
//     for i in 0..10 {
//         let a = Arc::clone(&a);
//         let handle = thread::spawn(move || {
//             a.send(i);
//         });
//         handles.push(handle);
//     }
//     for i in 0..10 {
//         let a = Arc::clone(&a);
//         let handle = thread::spawn(move || {
//             let c = a.receive();
//             println!("{:?}", c);
//         });
//         handles.push(handle);
//     }
//     for h in handles {
//         h.join().unwrap();
//     }
//     let x: MaybeUninit<u32> = MaybeUninit::uninit();
//     unsafe {
//         println!("{:?}", x.assume_init_read());
//     }
// }

// const EMPTY: u8 = 0;
// const READY: u8 = 1;
// const READING: u8 = 2;
// const WRITING: u8 = 3;

// pub struct OneShotChannel<T> {
//     message: UnsafeCell<MaybeUninit<T>>,
//     state: AtomicU8,
// }

// pub struct OneChannel<T> {
//     message: UnsafeCell<MaybeUninit<T>>,
//     state: AtomicU8,
// }

// unsafe impl<T> Sync for OneChannel<T> where T: Send {}

// impl <T> OneChannel<T> {
// pub fn new() -> Self {
//     Self {
//         message:UnsafeCell::new(MaybeUninit::uninit()),
//         state:AtomicU8::new(EMPTY),
//     }
// }
// pub fn split<'a>(&'a mut self) -> (Sender<'a T>,Reciever<'a,T>){
//     *self = Self::new();
//     (Sender {channel:self},Reciever{channel:self})

// }
// }

// pub struct Sender<'a,T> {
//     channel: &'a OneChannel<T>,
// }
// pub struct Reciever<'a,T> {
//     channel: &'a OneChannel<T>,
// }
// impl<T> Sender<T> {
//     pub fn send(&self, message: T) {
//         unsafe {
//             (*self.channel.message.get()).write(message);
//         }

//         self.channel
//             .state
//             .store(READY, std::sync::atomic::Ordering::Release);
//     }
// }

// impl<T> Reciever<T> {
//     pub fn receive(&self) -> T {
//         if self
//             .channel
//             .state
//             .compare_exchange(
//                 READY,
//                 READING,
//                 std::sync::atomic::Ordering::Relaxed,
//                 std::sync::atomic::Ordering::Relaxed,
//             )
//             .is_err()
//         {
//             panic!("No message Available");
//         }
//         unsafe { (*self.channel.message.get()).assume_init_read() }
//     }
//     pub fn is_ready(&self) -> bool {
//         self.channel.state.load(std::sync::atomic::Ordering::Acquire) == READY
//     }
// }

// impl<T> Drop for OneChannel<T> {
//     fn drop(&mut self) {
//         if *self.state.get_mut() == 1 {
//             unsafe {
//                 self.message.get_mut().assume_init_drop();
//             }
//         }
//     }
// }

// unsafe impl<T> Sync for OneShotChannel<T> where T: Send {}

// impl<T> OneShotChannel<T> {
//     pub fn new() -> Self {
//         Self {
//             message: UnsafeCell::new(MaybeUninit::uninit()),
//             state: AtomicU8::new(EMPTY),
//         }
//     }
//     pub fn send(&self, message: T) {
//         if self
//             .state
//             .compare_exchange(
//                 EMPTY,
//                 WRITING,
//                 std::sync::atomic::Ordering::Relaxed,
//                 std::sync::atomic::Ordering::Relaxed,
//             )
//             .is_err()
//         {
//             panic!("Channel is not empty");
//         }

//         unsafe {
//             (*self.message.get()).write(message);
//         }

//         self.state
//             .store(READY, std::sync::atomic::Ordering::Release);
//     }
//     pub fn is_ready(&self) -> bool {
//         self.state.load(std::sync::atomic::Ordering::Acquire) == READY
//     }
//     pub fn receive(&self) -> T {
//         if self
//             .state
//             .compare_exchange(
//                 READY,
//                 READING,
//                 std::sync::atomic::Ordering::Relaxed,
//                 std::sync::atomic::Ordering::Relaxed,
//             )
//             .is_err()
//         {
//             panic!("No message Available");
//         }
//         unsafe { (*self.message.get()).assume_init_read() }
//     }
// }

// impl<T> Drop for OneShotChannel<T> {
//     fn drop(&mut self) {
//         if *self.state.get_mut() == READY {
//             unsafe {
//                 (*self.message.get()).assume_init_drop();
//             }
//         }
//     }
// }

// pub fn oc() {
//     thread::scope(|s| {
//         let (sender,reciever) = onechannel();
//             let t = thread::current();
//         s.spawn(move || {
//             sender.send("hello_world");
//             t.unpark();
//         });
//         while !reciever.is_ready() {
//             thread::park();
//         }
//         println!("{:?} hh",reciever.receive() );
//     })

// }

use std::{
    cell::UnsafeCell,
    marker::PhantomData,
    mem::MaybeUninit,
    sync::atomic::AtomicBool,
    thread::{self, Thread},
    time::Duration,
};

struct Channel<T> {
    value: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}
struct Sender<'a, T> {
    receiving_thread: Thread,
    channel: &'a Channel<T>,
}
struct Receiver<'a, T> {
    channel: &'a Channel<T>,
    _no_data: PhantomData<*const ()>,
}

impl<T> Sender<'_, T> {
    pub fn send(self, message: T) {
        unsafe {
            (*self.channel.value.get()).write(message);
        }
        self.channel
            .ready
            .store(true, std::sync::atomic::Ordering::Release);
        self.receiving_thread.unpark();
    }
}

impl<T> Receiver<'_, T> {
    pub fn receive(self) -> T {
        while !self
            .channel
            .ready
            .swap(false, std::sync::atomic::Ordering::Acquire)
        {
            thread::park();
        }
        unsafe { (*self.channel.value.get()).assume_init_read() }
    }
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            value: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }
    pub fn split<'a>(&'a mut self) -> (Sender<'a, T>, Receiver<'a, T>) {
        *self = Self::new();
        (
            Sender {
                channel: self,
                receiving_thread: thread::current(),
            },
            Receiver {
                channel: self,
                _no_data: PhantomData,
            },
        )
    }
}

unsafe impl<T> Sync for Channel<T> where T: Send {}
impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe {
                (*self.value.get_mut()).assume_init_drop();
            }
        }
    }
}
pub fn oc() {
    let mut channel = Channel::new();
    thread::scope(|s| {
        let (sender, reciever) = channel.split();
        s.spawn(move || {
            sender.send("hello_world");
        });

        println!("{:?}", reciever.receive())
    });
}

