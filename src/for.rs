

fn handle_if() -> i32{
    let a = 0;
    if a > 0 {
        println!("{}", a);
    } else if a < 0 {
        println!("{}", a);
    } else {
        println!("a is 0");
    }

    let x = if a > 0{
        1
    } else {
        2
    };
    x
}

fn handle_loop() {
    loop {
        println!("hello world");
    }
}

fn handle_for() {
    let array = [1, 2, 3, 4, 5];
    for item in array.iter() {
        println!("{}", item)
    }
    for i in 1..100 {
        println!("{}", i)
    }
}


fn main() {
    // handle_if();
    // handle_loop();
    handle_for();
}