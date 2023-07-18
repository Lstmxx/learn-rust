use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::num::ParseIntError;


fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    let mut f = File::open("hello.txt")?;
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    let n1 = n1_str.parse::<i32>();
    let n2 = n2_str.parse::<i32>();
    Ok(n1.unwrap() * n2.unwrap())
}

fn main() {
    let s = read_username_from_file();
    match s {
        Ok(s) => println!("file content {s}"),
        Err(err) => println!("error {err}"),
    }

    let result = multiply("10", "2");
    assert_eq!(result, Ok(20));

    let result = multiply("t", "2");
    assert_eq!(result.unwrap(), 8);

    println!("Success!");
}
