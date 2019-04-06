/**
rust的内存管理方式：在编译期确定变量的生命周期，并自动释放内存
栈内存: 先进后出；从固定的位置取出，并且内存大小是已知固定的，所以速度更快
堆内存: 大小未知或大小可变，栈保存着指向堆内存的

规则：
值的所有者变量只有一个
当变量离开作用域，值被释放
简单的说：指向堆内存的指针只有一个
move
copy

在变量作为函数的参数，及将变量作为函数的返回值，也有所有权改变的情况
*/


fn handle_scope() {
    {
        let a = String::from("hello world");
    }
    println!("{}", a);
}

fn handle_move() {
    let s1 = String::from("hello world");
    let s2 = s1;
    println!("{}", s1);
}

fn handle_copy() {
    let x = 5;
    let y = x;
    println!("{}", y);
    let s1 = String::from("hello world");
    let s2 = s1.clone();
    println!("{}", s1);
}

fn handle_move_fn_arg(val: String) {
    println!("{}", val);
}

fn main() {
    let s1 = String::from("hello world");
    handle_move_fn_arg(s1);
    println!("{}", s1);
}
