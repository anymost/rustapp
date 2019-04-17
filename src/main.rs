// 引用的生命周期，主要涉及到引用
// 借助借用检查器，rust认为一个变量引用的另外一个变量的生命周期必须比自己长，不然就报错。
// 生命周期注解，不会修改引用的生命周期，而是指出引用的生命周期
// &str的生命周期是全局的




// fn invalid_reference() {
//     let r;
//     {
//         let x = 5;
//         r = &x;
//     }
//     println!("{}", r);
// }

fn valid_reference() {
    let x = 5;
    let r = &x;
    println!("{}", r);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn main() {
    // let a = "hello";
    // let b = "world";
    // let c = longest(a, b);
    // println!("{}", c);

    // let a = "hello";
    // {
    //     let b = "world";
    //     let c = longest(a, b);
    //     println!("{}", c);
    // }

    // let a = "hello";
    // let c;
    // {
    //     let b = String::from("hello world");
    //     c = longest(a, b.as_str());
    // }
    // println!("{}", c);
}