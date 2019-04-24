// box允许将一个值放在堆上，然后在栈上创建一个指针指向它。
// 当离开box的作用域时，box以及它指向堆上的数据会立即释放
// 适用场景
// 1 适用于大小可变或未知的数据
// 2 有大量数据，希望在不拷贝的情况下转移所有权
// 3 有一个值关注它是否实现了某个trait，而不关注类型

//enum List {
//    Cons(i32, List),
//    Nil
//}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}

use List::{Cons, Nil};

fn base_box() {
    let a = Box::new(1);
    println!("{}", a);
}

//fn recursive_box() {
//    let b = Cons(20, Cons(30, Cons(40, Cons(50, Cons(60, Nil)))));
//}

fn recursive_box() {
    let b = Cons(10,
        Box::new(Cons(
                     20,
            Box::new(Cons(
                        30,
                Box::new(Cons(
                        40,
                    Box::new(Cons(
                        50,
                        Box::new(Nil)))))))
        )
        )
    );
    println!("{:?}", b);
}

fn main() {
    // ase_box();
    recursive_box();
}