struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    // 关联函数
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle{
            width,
            height
        }
    }

    fn area(&self) -> u32{
        self.width * self.height
    }

    fn is_wider(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }
}

fn main() {
    let rectangle = Rectangle::new(20, 30);
    let rectangle2 = Rectangle::new(40, 50);
    println!("{}", rectangle.area());
    println!("{}", rectangle.is_wider(&rectangle2));
}