use num::complex::Complex;


fn use_complex() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im);
}

fn char_and_bool() {
    // Rust 的字符只能用 '' 来表示， "" 是留给字符串的
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小", std::mem::size_of_val(&x));

    // 单元类型()，可以理解为一种占位但是不占内存的类型
    println!("{}", i8::MAX)
}

fn express() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("the value of y is {}", y);
}

fn main() {
    let guess: i32 = "42".parse().expect("not a number");
    println!("guess {:?}", guess);
    // 溢出
    let a: u8 = 255;
    let b: u8 = a.wrapping_add(20);
    
    println!("b: {:?}", b);
    // 精度问题，在单精度f32下，0.1+0.2是等于0.3的，但在f64精度下，因为精度高了，导致更多的小数，而小数点后面发生了一些微小的变化
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!(" 0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!(" 0.3: {:x}", (abc.2).to_bits());

    println!("xyz (f64)");
    println!(" 0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!(" 0.3: {:x}", (xyz.2).to_bits());

    assert!(abc.0 + abc.1 == abc.2);
    // assert!(xyz.0 + xyz.1 == xyz.2);

    // NaN
    let x = (-42.0_f32).sqrt();
    // NaN之间不相等
    // assert_eq!(x, x);

    if x.is_nan() {
        println!("x is NaN");
    }

    let twenty = 20;
    let twenty_one: i32 = 21;

    let twenty_two = 22i32;

    // 只有同样类型，才能运算
    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    // 较长的数字可以用_来分割
    let one_million:i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];
    // 控制显示小数为2位
    println!("{:.2}", forty_twos[0]);

    // 位运算
    let c = 2;
    let d = 3;

    println!("c & d value is {}", c & d);
    println!("c | d value is {}", c | d);
    println!("c ^ d value is {}", c ^ d);

    println!("!d value is {}", !d);

    println!("(c << d) value is {}", c << d);
    println!("(c >> d) value is {}", c >> d);

    let mut c = c;
    c <<= d;
    println!("(c << d) value is {}", c);

    // 生成连续数值
    for i in 1..=5 {
        println!("i is {}", i);
    }

    for i in 'A'..='Z' {
        println!("i is {}", i);
    }

    use_complex();

    char_and_bool();

    express();
}
