//  RefCell允许在不可变引用的情况下修改内部值
// 通常，借用检查发生在编译时，没有性能损耗，如果检查不通过，则编译失败
// RefCell的检查发生在运行时，有一定性能损耗，检查不通过，会触发panic
// 内部可变性：不可变的值可变借用
// RefCell::new(v).borrow RefCell::new(v).borrow_mut()
// RefCell负责维护Ref和RefMut两个智能指针，其包含引用计数，其也遵守借用规则，一个可变引用或多个不可变引用
// Rc 和 RefCell可配合使用

use std::cell::RefCell;
use std::rc::Rc;

use List::{Cons, Nil};

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    value: usize,
    max: usize,
    messenger: &'a T,
}

impl<'a, T> LimitTracker<'a, T> where T: Messenger {
    fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        return LimitTracker {
            value: 0,
            max,
            messenger,
        };
    }
    fn set_value(&mut self, value: usize) {
        self.value = value;
        let percent = self.value as f64 / self.max as f64;
        match percent {
            0.0...0.75 => {
                self.messenger.send("good");
            }
            0.75...0.9 => {
                self.messenger.send("caution");
            }
            0.9...1.0 => {
                self.messenger.send("warning");
            }
            _ => {
                self.messenger.send("danger");
            }
        }
    }
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    let mut v = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&v), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(20)), Rc::clone(&a));
   *v.borrow_mut() += 10;
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

}
//
//#[cfg(test)]
//mod tests {
//    use std::cell::RefCell;
//
//    use super::*;
//
//    struct MockMessenger {
//        messages: RefCell<Vec<String>>
//    }
//
//    impl MockMessenger {
//        fn new() -> MockMessenger {
//            return MockMessenger {
//                messages: RefCell::new(vec![])
//            };
//        }
//    }
//
//    impl Messenger for MockMessenger {
//        fn send(&self, msg: &str) {
//            self.messages.borrow_mut().push(String::from(msg));
//        }
//    }
//
//    #[test]
//    fn test_send_75_message() {
//        let messenger = MockMessenger::new();
//        let mut tracker = LimitTracker::new(&messenger, 1);
//        tracker.set_value(0.75 as usize);
//    }
//}
//
//#[cfg(test)]
//mod tests {
//    use std::cell::RefCell;
//
//    use super::*;
//
//    struct MockMessenger {
//        messages: RefCell<Vec<String>>
//    }
//
//    impl MockMessenger {
//        fn new() -> MockMessenger {
//            return MockMessenger {
//                messages: RefCell::new(vec![])
//            };
//        }
//    }
//
//    impl Messenger for MockMessenger {
//        fn send(&self, msg: &str) {
//            let mut ref_one = self.messages.borrow_mut();
//            let mut ref_two = self.messages.borrow_mut();
//            ref_one.push(String::from(msg));
//            ref_two.push(String::from(msg));
//        }
//    }
//
//    #[test]
//    fn test_send_75_message() {
//        let messenger = MockMessenger::new();
//        let mut tracker = LimitTracker::new(&messenger, 1);
//        tracker.set_value(0.75 as usize);
//    }
//}
