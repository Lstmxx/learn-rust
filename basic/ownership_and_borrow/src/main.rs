fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() 在字符串后追加字面值
    println!("{}", s);

    // 存放在堆的数据，通过变量绑定，上一个绑定的会失效。
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, ?", s1);

    // 在rust中浅拷贝只会发生在存放在栈的数据上
    let x = 5;
    let y = &x;
    assert_eq!(5, *y);
    println!("y is {}", y);

    // 切片时候要注意utf8的问题，一个中文在utf8中占用三个字节
    let s = "gsc";
    let a = &s[0..2];
    println!("{}",a);

    let g = [1, 2, 3, 4, 5];

    let slice = &g[1..3];
    println!("{:?}", slice);
}
