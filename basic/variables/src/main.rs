struct Struct {
    t: i32
}

fn main() {
    // mut 关键字可以使变量变得可修改
    let mut x = 5;
    println!("the value of x is: {}", x);
    x = 6;
    println!("the value of x is: {}", x);
    // 变量解构
    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);
    b = true;
    assert_eq!(a, b);
    
    //结构式赋值
    let (q, w, e , r, t);

    (q, w) = (1, 2);

    Struct { t, .. } = Struct { t: 5 };

    [e, .., r, _] = [1, 2, 3, 4, 5];

    assert_eq!([1, 2, 1, 4, 5], [q, w, e , r, t]);
    
    // 常量
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {:?}", MAX_POINTS);

    // 变量遮蔽，使用let来声明相同的变量会进行覆盖，和mut不同的是，
    // 使用let声明相同的变量会再进行一次内存分配，而mut则不会，所以mut是不能修改变量类型的，而通过let来覆盖的话则没问题
    let i = 5;
    let i = i + 1;

    {
        let i = i * 2;
        println!("The value of x in the inner scope is: {}", i);
    }
    println!("The value of x is: {}", i);
}
