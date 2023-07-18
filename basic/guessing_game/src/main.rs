use std::io;

fn main() {
    println!("猜数");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("无法读取捏~");

    println!("你猜的是：{}", guess);
    
    // let secret_number = rand::
}
