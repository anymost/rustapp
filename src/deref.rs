// Deref trait 在执行解引用(*)的时候调用
// *y => *(y.deref())
// 解引用强制多态：将实现了Deref的类型的引用转换为原始类型

use std::ops::Deref;

fn base() {
    let a = 1;
    let b = &a;
    println!("{}", b);
}

fn dereference_box() {
    let a = 1;
    let b = Box::new(a);
    assert_eq!(1, *b);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn test_deref() {
    let a = 1;
    let my_box = MyBox::new(a);
    assert_eq!(a, 1);
    assert_eq!(*my_box, 1);
}

fn say_hello(m: &str) {
    println!("{}", m);
}

fn main() {

    let str_box = MyBox::new(String::from("hello world"));
    // &MyBox<String> => &String => &str
    say_hello(&str_box);
}