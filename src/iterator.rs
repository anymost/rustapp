use std::collections::hash_map::HashMap;

/**
 * 迭代器是惰性的，不消费不会有任何损耗
 * iter() 不可变引用
 * into_iter() 所有权
 * iter_mut() 可变引用
 * 消费适配器用于消费迭代器,一些消费适配器会获取迭代器的所有权
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

fn consume_iterator() {
    let vector = vec![1, 2, 3, 4, 5];
    let mut it = vector.iter();
    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next(), Some(&3));
    assert_eq!(it.next(), Some(&4));
    assert_eq!(it.next(), Some(&5));
    assert_eq!(it.next(), None);
}

// 消费适配器
fn consume_adapter() {
    let vector = vec![1, 2, 3, 4, 5, 6];
    let it = vector.iter();
    // sum 会获取迭代器所有权
    let sum: i32 = it.sum();
    println!("{}", sum);
}

// 迭代器适配器
fn iterate_adapter() {
    let vector = vec![1, 2, 3, 4, 5, 6];
    let new_vector: Vec<i32> = vector.iter().map(|x| x + 1).collect();
    println!("{:?}", new_vector);
}

fn test_iter() {
    iterate_array();
    iterate_vector();
    iterate_map();
    consume_adapter();
    iterate_adapter();
    let flag = &&4;
    let filter_and_plus_one = |vector: Vec<i32>| vector.iter().filter(|x| x < flag).map(|x| x + 1).collect();
    let new_vec: Vec<i32> = filter_and_plus_one(vec![1, 2, 3, 4, 5]);
    println!("{:?}", new_vec);
}

struct Counter {
    count: i32
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0
        }
    }
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 10 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));

    let new_counter = Counter::new().map(|v| v + 1).collect();
    println!("{}", new_counter);
}