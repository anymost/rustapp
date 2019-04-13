/**
 * 泛型单态化：在编译的时候，确定泛型中具体的类型，所以不会有性能损耗
 */
#[derive(Debug)]
struct User<T> {
    name: T,
    age: i32
}

#[derive(Debug)]
enum Car<T> {
    BMW(T),
    BENZ
}

impl<T> User<T> {
    fn get_name(&self) -> &T {
        &self.name
    }
}

fn handle_generic_struct() {
    let user = User{
        name: "jack",
        age: 20
    };
    let user2 = User{
        name: 30,
        age: 20
    };
    println!("{:?}", user);
    println!("{:?}", user2);
    println!("{}", user.get_name());
}

fn handle_generic_enum() {
    let benz: Car<&str> = Car::BMW("Germany");
    println!("{:?}", benz); 
}
