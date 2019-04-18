// 引用的生命周期，主要涉及到引用
// 借助借用检查器，rust认为一个变量引用的另外一个变量的生命周期必须比自己长，不然就报错。
// 生命周期注解，不会修改引用的生命周期，而是指出引用的生命周期
// &str属于静态生命周期，也就是全局的
/**
 * 生命周期省略规则
 * 参数的生命周期为输入生命周期，返回值的生命周期为输出生命周期
 * 1 每一个为引用的参数都有一个生命周期参数
 * 2 如果只有一个输入生命周期参数，那么它被赋予所有的输出生命周期参数
 * 3 如果有多个输入生命周期参数，其中一个为&self或&mut self,那么它被赋予所有的输出生命周期参数
 * 
 * 借助上面的三条规则，来判断是否能确定一个函数中的所有生命周期参数，如果无法确定，那么就编译失败
 */




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



#[derive(Debug)]
struct User<'a> {
    name: &'a str,
    age: i32
}

fn origin(a: & str) -> & str {
    a
}

impl<'a> User<'a> {
    fn say_name(&self, v: &str) -> &str {
        self.name
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


    let s = String::from("hello");
    let s2 = String::from("world");
   
    let user = User{
        name: &s[..],
        age: 20
    };

    let val = user.say_name(&s2);
    println!("{}", val);
}