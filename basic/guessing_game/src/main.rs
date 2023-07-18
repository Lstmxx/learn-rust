use std::io;
use rand::Rng;

use std::cmp::Ordering;

fn main() {
    println!("猜数");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("无法读取捏~");

    println!("你猜的是：{}", guess);
    
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("神秘数字是！！！： {}", secret_number);

    let guess:u32 = guess.trim().parse().expect("转换失败捏");


    match guess.cmp(&secret_number) {
        Ordering::Less => println!("small !"),
        Ordering::Greater => println!("big !"),
        Ordering::Equal => println!("yes!"),
    }
}
