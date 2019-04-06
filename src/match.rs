#[derive(Debug)]
enum Gender {
    Man,
    Woman
}

#[derive(Debug)]
enum User {
    Musican(Gender),
    Teacher(String)
}

#[derive(Debug)]
enum Count {
    One,
    Two,
    Three,
    Four
}

fn match_gender(gender: Gender) {
    match gender {
        Gender::Man => {
            println!("man");
        },
        Gender::Woman => {
            println!("woman");
        }
    }
}

fn match_user(user: User) {
    match user {
        User::Musican(gender) => {
            println!("{:?}", gender);
        },
        User::Teacher(name) => {
            println!("{}", name)
        }
    }
}

fn match_option(gender: Option<Gender>) {
    match gender {
        Some(gender) => {
            println!("{:?}", gender);
        },
        None => {
            println!("None");
        }
    }
}

fn match_general(count: Count) {
    match count {
        Count::One => {
            println!("one");
        },
        Count::Two => {
            println!("two");
        },
        _ => {
            println!("other");
        }
    }
}

fn main() {
    // let woman = Gender::Man;
    // match_gender(woman);
    // let user = User::Musican(Gender::Man);
    // match_user(user);

    // let gender:Option<Gender> = Some(Gender::Man);
    // match_option(gender);

    let count = Count::Four;
    match_general(count);
}