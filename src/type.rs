/**
  rust为静态类型语言，即需要在编译器确定变量的类型
  基本类型:
      整型 -(2^n) ~ (2^n)-1 i8 i16 i32 i64 u8 u16 u32 u64 isize usize 这两个用于集合的索引
      浮点型 u32 u64
      bool
      char
  复合类型:
     元组 长度固定，类型可以不同
     数组 长度固定，类型必须相同
*/

fn handle_tuple() {
    let tuple = (1, 2.0, true);
    let (x, y, z) = tuple;
    println!("{} {} {}", x, y, z);
    let new_tuple = (1, 2.0, true);
    println!("{} {}", new_tuple.0, new_tuple.1)
}

fn handle_array() {
    let array:[i32;4] = [1, 2, 3, 4];
    println!("{}", array[0]);
}


fn main() {
    handle_tuple();
    handle_array();
}
