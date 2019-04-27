use std::cell::RefCell;
use std::rc::Rc;

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

}