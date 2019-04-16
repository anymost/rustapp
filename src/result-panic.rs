/**
 * panic!不可恢复，可以使用RUST_BACKTRACE=1来展开错误栈
 * unwarp()和 expect()会自动对Result值进行模式匹配，如果为Err，触发panic；如果不为Err，
 * 返回解构后的Ok值
 * 对Result调用？，如果有Err，会自动返回；否则，返回解构后的Ok值
 */

use std::fs::File;
use std::io::Read;
use std::io::Error;

fn match_result() {
    let file = File::open("./src/enum.rs");
    let mut s = String::new();
    match file {
        Ok(mut f) => {
            f.read_to_string(&mut s);
        },
        Err(error) => {
            panic!(error);
        }
    }
    println!("{}", s);
}

fn handy_result() {
    // let mut f = File::open("./src/enum.rs").unwrap();
    let mut f = File::open("./enum.rs").expect("not found");
    let mut s = String::new();
    f.read_to_string(&mut s);
    println!("{}", s);
}

fn return_result() -> Result<File, Error> {
    let f = File::open("./src/enum.rs");
    match f {
        Ok(file) => {
            Ok(file)
        },
        Err(e) => {
            Err(e)
        }
    }
}

fn read_file() -> Result<String, Error> {
    let mut s = String::new();
    File::open("./src/enum.rs")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let s = read_file().unwrap();
    println!("{}", s);
}