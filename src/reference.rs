/**
借用：使用引用作为函数的参数

引用规则：
要么多个不可变引用，要么一个可变引用
*/


fn handle_reference(val: &String) {
    println!("{}", val)
}

fn handle_mutable_reference() {
    let mut s1 = String::from("hello world");
    let s2 = &mut s1;
    s2.push_str(" xixi");
    println!("{}", s2);
}

fn handle_immutable_reference() {
    let s1 = String::from("hello world");
    let s2 = &s1;
    let s3 = &s1;
}

fn dangling_reference() -> &String {
    let s = String::from("hello world");
    return &s;
}




fn main() {
//    let s = String::from("hello world");
//    handle_reference(&s);
    handle_mutable_reference();
    handle_immutable_reference();
}