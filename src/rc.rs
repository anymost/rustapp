use std::rc::Rc;

use List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn test_rc() {
    let a = Rc::new(Cons(10, Rc::new(Cons(20, Rc::new(Cons(30, Rc::new(Cons(40, Rc::new(Nil)))))))));
    println!("{}", Rc::strong_count(&a));
    let b = Rc::clone(&a);
    println!("{}", Rc::strong_count(&a));
    {
        let c = Rc::clone(&a);
        println!("{}", Rc::strong_count(&a));
    }
    println!("{}", Rc::strong_count(&a));
}

fn main() {
    test_rc();
}
