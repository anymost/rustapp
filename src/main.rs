//  RefCell允许在不可变引用的情况下修改内部值
// 通常，借用检查发生在编译时，没有性能损耗，如果检查不通过，则编译失败
// RefCell的检查发生在运行时，有一定性能损耗，检查不通过，会触发panic
// 内部可变性：不可变的值可变借用

pub trait Messenger {
	fn send(&self, msg: &str);
}
pub struct LimitTracker<'a, T: 'a+Messenger> {
	value: usize,
	max: usize,
	messenger: &'a T
}
impl<'a, T> LimitTracker<'a, T> where T : Messenger {
	fn new(messenger: &T, max: usize) -> LimitTracker<T> {
		return LimitTracker{
			value: 0,
			max,
			messenger
		}
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

#[cfg(test)]
mod tests {
	use super::*;
	use std::cell::RefCell;

	struct MockMessenger {
		messages: RefCell<Vec<String>>
	}
	impl MockMessenger {
		fn new() -> MockMessenger {
			return MockMessenger{
				messages: RefCell::new(vec![])
			}
		}
	}
	impl Messenger for MockMessenger {
		fn send(&self, msg: &str) {
			self.messages.borrow_mut().push(String::from(msg));
		}
	}

	#[test]
	fn test_send_75_message() {
		let messenger = MockMessenger::new();
		let mut tracker = LimitTracker::new(&messenger, 1);
		tracker.set_value(0.75 as usize);
	}
}
