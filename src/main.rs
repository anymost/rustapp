// Drop Trait 在值将要离开作用域的时候执行

#[derive(Debug)]
struct User {
    name: String,
    age: i32
}

impl Drop for User {
    fn drop(&mut self) {
        println!("before drop");
        println!("{:?}", self);
    }
}

fn auto_drop() {
    {
        let user = User {
            name: String::from("jack"),
            age: 20
        };
    }
    println!("after drop");
}

fn manual_drop() {
    {
        let user = User {
            name: String::from("jack"),
            age: 20
        };
        drop(user);
        println!("after drop");
    }
}

fn main() {
    manual_drop()
}