use std::collections::HashMap;

fn init_map() {
    let mut map: HashMap<i32, String> = HashMap::new();
    map.insert(1, String::from("hello"));
    map.insert(2, String::from("world"));
    map.insert(3, String::from("beauty"));
    
    let value = map.get(&1);

    match value {
        Some(v) => {
            println!("{}", v);
        }, 
        None => {
            println!("none");
        }
    }

    for (key, val) in &map {
        println!("{}: {}", key, val)
    }

    map.entry(4).or_insert(String::from("jack"));

    println!("{:?}", map);
}

fn main() {
    init_map();
}