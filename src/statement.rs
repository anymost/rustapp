/**
如何区分表达式和语句: 表达式会返回一个值
*/

fn complex_expression() {
    let x = {
        let y = 1;
        y + 1
    };
    println!("{}", x)
}

fn auto_return() -> i32 {
    let x = 1;
    x + 1
}

fn main() {
    let y = auto_return();
    println!("{}", y);
}