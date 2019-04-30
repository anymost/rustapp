// 管道是单所有权，一旦一个值被写入这个管道，将无法再使用
// 共享内存是躲所有权，多个线程可以访问同一块内存地址空间

use std::sync::Mutex;
use std::rc::Rc;
use std::thread;


fn base() {
    let mutex = Mutex::new(5);
    {
        let mut v = mutex.lock().unwrap();
        *v = 6;
    }
    let v = mutex.lock().unwrap();
    println!("{}", v);
}

fn multi_thread() {
    let mut handles = vec![];
    let mutex = Rc::new((Mutex::new(0)));
    for i in 0..5 {
        let m = Rc::clone(&mutex);
        let handle = thread::spawn(move ||{
            let mut v = m.lock().unwrap();
            *v = *v + 1;
        });
        handles.push(handle);
    }
    for x in handles {
        x.join();
    }
}

fn main() {
    multi_thread();
}