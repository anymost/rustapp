// 管道是单所有权，一旦一个值被写入这个管道，将无法再使用
// 共享内存是多所有权，多个线程可以访问同一块内存地址空间
// Rc无法保证在多线程环境下是原子操作
// Mutex同样提供了内部可变性


use std::sync::{Mutex, Arc};
use std::thread;


fn base() {
    let mutex = Mutex::new(10);
    let mut v = mutex.lock().unwrap();
    *v = 1;
    println!("{}", v);
}

//fn multi_thread() {
//    let mut vector = vec![];
//    let mutex = Rc::new(Mutex::new(0));
//    for _ in 0..10 {
//        let m = Rc::clone(&mutex);
//        let handle = thread::spawn(move ||{
//            let mut v = m.lock().unwrap();
//            *v += 1;
//        });
//        vector.push(handle);
//    }
//    for v in vector {
//        v.join();
//    }
//    println!("{:?}", mutex);
//}

fn multi_thread() {
    let mutex = Arc::new(Mutex::new(5));
    let mut vector = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&mutex);
        let handle = thread::spawn(move || {
            let mut v = counter.lock().unwrap();
            *v += 1;
        });
        vector.push(handle);
    }
    for v in  vector {
        v.join();
    }
    println!("{:?}", mutex);
}

fn main() {
    multi_thread();
}