/**
宏
1 声明宏
2 过程宏
   自定义宏
   类属性宏
   类函数宏
*/

#[macro_export]
macro_rules! list {
    ($($x: expr), *) => {
        {
            let mut value = Vec::new();
            $(
                value.push($x);
            )*
            value
        }
    }
}

#[macro_export]
macro_rules! sum {
    ($($x: expr), *) => {
        {
            let mut sum = 0;
            $(
                sum += $x;
            )*
            sum
        }
    }
}
// 测试声明宏
fn test_declarative_macro() {
    let list = list![1, 2, 3, 4];
    println!("{:?}", list);

    let result = sum![1, 2, 3, 4];
    println!("{}", result);
}






fn main() {

}
