#[derive(Debug)]
enum Value {
    IntValue(i32),
    FloatValue(f64),
    BoolValue(bool),
    CharValue(char)
}

fn tuple_and_array() {
    let tuple: (i32, i32, i32) = (1, 2, 3);
    println!("{:?}", tuple);
    let array: [i32; 4] = [1, 2, 3, 4];
    println!("{:?}", array);
}

fn init_vector() {
    let vector = vec![1, 2, 3, 4];
    println!("{:?}", vector);
}

fn base_vector() {
    let mut vector: Vec<i32> = Vec::new();
    vector.push(20);
    vector.push(30);
    vector.push(40);
    vector.push(50);
    println!("{:?}", vector);
}

fn get_vector() {
    let vector = vec![1, 2, 3];
    let val = vector.get(0);
    let mut num: &i32 = &0;
    match val {
        Some(int_val) => {
            num = int_val;
        },
        None => {
            panic!("out of range");
        }
    }
    println!("{}", num);

    println!("{}", &vector[0])
}

fn iterate_vector() {
    let mut vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for val in &mut vector {
        *val += 10;
    }
    println!("{:?}", vector)
}

fn enum_vector() {
    let vector: Vec<Value> = vec![
        Value::IntValue(10),
        Value::FloatValue(1.0),
        Value::CharValue('a'),
        Value::BoolValue(true)
    ];
    println!("{:?}", vector);
}

fn main() {
    tuple_and_array();
    base_vector();
    init_vector();
    get_vector();
    iterate_vector();
    enum_vector();
}