/**
 * 
 * 闭包不需要定义参数类型，如果两次调用闭包参数类型不同会报错
 * 闭包能够捕获当前环境的变量，函数则不能
 */
use std::{thread, time::Duration, collections::hash_map::HashMap};

fn base_closure() {
    let println = |val: i32| val;
}

// fn invalid_closure() {
//     let print = |val| val;
//     print("hello");
//     print(2);
// }

struct Cacher<T>
    where T: Fn(i32) -> i32
 {
     calculate: T,
     val: Option<i32>
}



impl<T> Cacher<T>
    where T: Fn(i32) -> i32
 {
    fn new(calculate: T) -> Cacher<T> {
        Cacher{
            calculate,
            val: None
        }
    }

    fn value(&mut self, arg: i32) -> i32 {
        match self.val {
            Some(v) => {
                arg
            },
            None => {
                let v = (self.calculate)(arg);
                println!("calculate...");
                self.val = Some(v);
                v
            }
        }
    }
}

fn capture_value() {
    let x = 1;
    let is_equal = |y| x == y;
    let z = 2;
    println!("{}", is_equal(z));
}

fn main() {
    // let expensive_calculate = |x| {
    //     thread::sleep(Duration::from_secs(2));
    //     x
    // };
    // let mut cacher = Cacher::new(expensive_calculate);
    // let mut val = cacher.value(1);
    // println!("{}", val);
    // val = cacher.value(2);
    // println!("{}", val);
    capture_value();
    
}