// 模式匹配所有的情况
/**
1 match
2 if let
3 while left
4 for
5 let
6 函数参数
*/
// 模式分为可反驳和不可反驳
// 不可反驳：能够接受任何值 let定义 函数参数 for
// 可反驳： 只能接受特定值 if let, while let  if let Some(x) = hello() ，hello返回值只能为Some,不能为None
// 不可反驳与可反驳不能混用

fn test() -> Option<i32> {
    Some(1)
}

fn base() {
    let a = 1;
    if let Some(x) = test() {
        println!("{}", x);
    }
}

fn multi_match() {
    let x = 5;
    match x {
        5 | 10 => {
            println!("{}", x);
        }
        _ => {
            println!("other");
        }
    }

    let y = 'a';
    match y {
        'a'...'g' => {
            println!("{}", y);
        }
        _ => {
            println!("other");
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn match_struct() {
    let Point { x: a, y: b } = Point { x: 1, y: 2 };
    println!("{} {}", a, b);
    let p = Point { x: 1, y: 2 };
    match p {
        Point { x: 1, y: 3 } => {
            println!("ok");
        }
        Point { x: 2, y: 1 } => {
            println!("hello");
        }
        Point { x, y } => {
            println!("all");
        }
    }
}

enum Money {
    One,
    Two(i32),
    Three(Point),
}

fn match_enum() {
    let v = Money::Two(20);
    match v {
        Money::One => println!("One"),
        Money::Two(x) => println!("{}", x),
        Money::Three(Point { x, y }) => println!("{} {}", x, y),
    }
}

fn match_ref() {
    let vector = vec![
        Point { x: 1, y: 2 },
        Point { x: 1, y: 2 },
        Point { x: 1, y: 2 },
        Point { x: 1, y: 2 },
    ];
    for (x, y) in vector.iter().map(|&Point { x, y }| x + y).enumerate() {
        println!("{} {}", x, y);
    }
}

fn match_ignore() {
    // -  用于忽略所有值
    // .. 用于忽略剩余值
    let _x = 5;
    let p = Some(5);
    match p {
        Some(_) => {
            println!("ignore");
        }
        _ => {
            println!("default");
        }
    }
    let v = (1, 2, 3, 4, 5);
    match v {
        (a, ..) => {
            println!("{}", a);
        }
    }
}

fn match_guard() {
    let a = Some(30);
    match a {
        Some(50) => println!("50"),
        Some(x) if x > 10 => {
            println!("x > 10")
        },
        Some(x) => {
            println!("{}", x);
        },
        None => {
            println!("none");
        }
    }
}

fn main() {
    // base();
    // multi_match();
    // match_struct();
    // match_enum();
    // match_ref();
    // match_ignore();
    match_guard();
}
