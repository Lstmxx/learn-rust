#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

// 元组结构体
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

// 单元结构体
#[derive(Debug)]
struct AlwaysEqual;
trait SomeTrait {
    
}

impl SomeTrait for AlwaysEqual {
    
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from(">>><<<"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("hhhhh@example.com");

    // 结构体更新语法，类似于ts中的...。需要注意所有权转移，像下面的代码user1中的username就发生了所有权转移
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{}", user1.sign_in_count);

    println!("{:?}", user2);

    let f1 = File {
        name: String::from("hhh.jpg"),
        data: Vec::new(),
    };

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{:?}", black);
    println!("{:?}", origin);

    let subject = AlwaysEqual;
    
}
