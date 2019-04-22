use std::collections::hash_map::HashMap;
/**
 * 迭代器是惰性的，不消费不会有任何损耗
 *
  
 */


fn iterate_array() {
    let array: [i32; 6] = [1, 2, 3, 4, 5, 6];
    for item in array.iter() {
        println!("{}", item);
    }
}

fn iterate_vector() {
    let vector = vec![1, 2, 3, 4, 5, 6];
    for item in vector.iter() {
        println!("{}", item);
    }
}

fn iterate_map() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(1, 1);
    map.insert(2, 2);
    map.insert(3, 3);
    map.insert(4, 4);
    map.insert(5, 5);
    map.insert(6, 6);

    for (key, value) in map.iter() {
        println!("{} {}", key, value);
    }
}

fn main() {
    iterate_array();
    iterate_vector();
    iterate_map();
}