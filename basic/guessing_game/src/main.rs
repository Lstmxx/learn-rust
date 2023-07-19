use std::io;
use rand::Rng;

use std::cmp::Ordering;

fn main() {
    println!("猜数");
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess = String::new();
        match io::stdin().read_line(&mut guess)  {
            Ok(str) => str,
            Err(_) => {
                println!("无法读取捏~");
                continue;
            },
        };
        println!("你猜的是：{}", guess);
    
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("转换失败捏");
                continue;
            },
        };
    
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("small !"),
            Ordering::Greater => println!("big !"),
            Ordering::Equal => {
                println!("yes!");
                break;
            },
        }
    }

}
