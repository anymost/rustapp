/**
切片：切片使我们能引用部分内容
对于String类型，其切片s[..]为str类型
对于字符串的字面量，其为&str类型
*/

fn print_str(val: &str) {
    println!("{}", val)
}

fn array_slice() {
    let array = [1, 2, 3, 4, 5];
    let array2 = &array[..];
    for val in array2.iter() {
        println!("{}", val);
    }
}

fn main() {
    let s1 = String::from("hello world");
    print_str(&s1[..]);
    let s2 = "hello world";
    print_str(s2);
    array_slice();
}