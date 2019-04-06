
#[derive(Debug)]
enum Gender {
    MAN,
    WOMAN
}

#[derive(Debug)]
enum User {
    Artist(String),
    Sporter{name: String, age: i32},
    Teacher(i32, i64)
}

fn basich_enum() {
    let mut gender = Gender::MAN;
    println!("{:?}", gender);
    gender = Gender::WOMAN;
    println!("{:?}", gender);
}

fn type_enum() {
    let mut user: User = User::Artist(String::from("pino"));
    println!("{:?}", user);
    user = User::Teacher(20, 30);
    println!("{:?}", user);
    
}

impl Gender {
    fn new(isMan: bool) -> Gender {
        if isMan {
            Gender::MAN
        } else {
            Gender::WOMAN
        }
    }
}

fn option_enum() {
    let mut val: Option<i32> = None;
    val = Some(20);

}

fn is_Ok() -> Option<bool> {
    Some(true)
}

fn main() {
    // type_enum();
    // let gender = Gender::new(true);
    // println!("{:?}", gender);
    option_enum();
}