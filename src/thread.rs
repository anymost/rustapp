// 运行时：指二进制文件中由语言本身提供的代码
// 使用join,阻塞主线程，等待子线程执行完成
use std::thread;
use std::time::Duration;

fn base() {
    thread::spawn(||{
        for i in 1..10 {
            println!("other {}", i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    for i in 1..10 {
        println!("main {}", i);
        thread::sleep(Duration::from_secs(1));
    }
}

fn join() {
    let handle = thread::spawn(||{
        for i in 0..20{
            println!("other {}", i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    handle.join().unwrap_err();

    for i in 0..10{
        println!("main {}", i);
        thread::sleep(Duration::from_secs(1));
    }
}

fn move_thread() {
    let vector = vec![1, 2, 3, 4, 5];
    let handle = thread::spawn(move ||{
        println!("{:?}", vector);
    });
    handle.join().unwrap();
}

fn main() {
    // base();
    // join();
    move_thread();
}