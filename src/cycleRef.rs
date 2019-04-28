use std::cell::RefCell;
use std::rc::Rc;


#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil
}

use List::{Cons, Nil};

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, v) => Some(v),
            Nil => None
        }
    }
}

fn main() {
    let a = Rc::new(Cons(10, RefCell::new(Rc::new(Nil))));
    println!("{}", Rc::strong_count(&a));
    // 解引用强制多态
    println!("{:?}", a.tail());
    let b = Rc::new(Cons(20, RefCell::new(Rc::clone(&a))));
    println!("{}", Rc::strong_count(&a));
    println!("{}", Rc::strong_count(&b));
    println!("{:?}", b.tail());
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("{}", Rc::strong_count(&a));
    println!("{}", Rc::strong_count(&b));
    // stack overflow
    println!("{:?}", a.tail());
}