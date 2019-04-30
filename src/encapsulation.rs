// 面向对象： 对象：数据+方法对应到rust中是struct和method
// 封装: rust的模块系统提供访问性的控制
// 继承: trait中默认的方法会所有impl该trait的结构体继承
// 多态: 不同的struct可以对trait实现不同的行为

mod user {
    pub struct User<'a> {
        name: &'a str,
        age: u8,
        address: &'a str
    }

    impl<'a> User<'a> {
        pub fn new(name: &'a str, age: u8, address: &'a str) -> User<'a> {
            User{
                name,
                age,
                address
            }
        }

        pub fn get_name(&self) -> &str {
            self.name
        }

        pub fn get_age(&self) -> u8 {
            self.age
        }

        pub fn get_address(&self) -> &str {
            self.address
        }
    }
}

use user::User;

pub mod duo_tai {
    pub trait Singer {
        fn say_something(&self) {
            println!("from Father")
        }
    }
}



fn encapsulation() {
    let user = User::new("jack", 20, "USA");
    println!("{}", user.get_name());
}

