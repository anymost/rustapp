// "".to_string() 和 String::from("")完全一样

fn init_string() {
    let v1 = "hello world".to_string();
    let v2 = String::from("hello world");
    println!("{}", v1 == v2);
}

fn push_string() {
    let mut s = String::from("hello");
    let more = "world";
    s.push_str(more);
    s.push('a');
    println!("{}", s)
}

fn add_string() {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    // s1的所有权被转移 &s2是&String类型，但是这里需要的&str,发生了解引用强制多态 
    let s3 = s1 + &s2;
    println!("{}", s3);
    let s4 = format!("{}{}", s2, s3);
    println!("{}", s4);
}

fn index_string() {
    let s = String::from("我是超人");
    println!("{}", s.len());
    println!("{}", &s[0..3]);
}

fn range_string() {
    let s = String::from("我是超人");
    for val in s.chars() {
        println!("{}", val);
    }
    for val in s.bytes() {
        print!("{}", val);
    }
}

fn main() {
    range_string();
}