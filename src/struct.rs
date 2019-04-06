
struct User {
    name: String,
    age: i32,
    gender: bool
}

struct Car(i32, i32, i32);

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}

fn init_struct() {
    let name = String::from("hello");
    let user = User {
        name,
        age: 20,
        gender: true
    };
}

fn generate_struct() {
    let user1 = User{
        name: String::from("jack"),
        age: 20,
        gender: true
    };
    let user2 = User {
        ..user1
    };
}

fn tuple_struct() {
    let benz = Car(1, 2, 3);
    println!("{}", benz.0)
}
fn area(rectangle: Rectangle) -> u32 {
    &rectangle.width * &rectangle.height
}

fn main() {
    tuple_struct();
    let rectangle = Rectangle{
        width: 20,
        height: 30
    };
    println!("{:?}", rectangle);
    let area = area(rectangle);
    println!("{}", area);
}
