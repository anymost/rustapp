/**
这里有两种不同的实现下面Screen struct的方式
ScreenOne: 允许vec中变量有不同的类型，只要他们都实现了Draw trait
ScreenTwo: vec中的变量必须是同一类型，这一类型需要实现Draw trait

这里的多态依然使用了鸭式辩形
*/
pub trait Draw{
    fn draw(&self);
}

struct ScreenOne {
    components: Vec<Box<dyn Draw>>
}

struct ScreenTwo<T> where T: Draw {
    components: Vec<T>
}

impl ScreenOne {
    fn run(&self) {
        for item in self.components.iter() {
            item.draw();
        }
    }
}

impl<T> ScreenTwo<T> where T: Draw {
    fn run(&self) {
        for item in self.components.iter() {
            item.draw();
        }
    }
}

struct Button {
    width: i32,
    height: i32
}

impl Draw for Button {
    fn draw(&self) {
        println!("width is {}, height is {}", self.width, self.height);
    }
}

struct Table {
    rows: i32,
    cols: i32
}

impl Draw for Table {
    fn draw(&self) {
        println!("rows is {}, cols is {}", self.rows, self.cols);
    }
}


fn main() {
    let screen = ScreenOne {
        components: vec![
            Box::new(Button{
                width: 20,
                height: 20
            }),
            Box::new(Table{
                rows: 20,
                cols: 20
            })
        ]
    };
    screen.run();
}
