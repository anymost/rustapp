mod content;

use content::Gender;
use content::User;

mod user {
    pub mod external {
        pub mod internal {
            pub const a: i32 = 1;
        }
    }
    pub fn say_hello() {
        println!("hello world");
        super::test_mod1();
    }
}

mod object {
    pub use crate::user::external;
    pub fn say_hi() {
        super::user::say_hello();
    }
}

fn test_mod1() {
    println!("this is module 1");
}

use user::say_hello;
use  crate::object::say_hi;

fn main() {
    user::say_hello();
    say_hello();
    say_hi();
    let gender = User {
        name:String::from("hello"),
        age: 20,
        address: String::from("China"),
        gender: Gender::Male,
    };
}