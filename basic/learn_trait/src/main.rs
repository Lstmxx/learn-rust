use std::{fmt::Display, process::Output};
use core::fmt::Debug;
use std::ops::Add;

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.content)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}

// 表示实现了Summary特征的数据类型都能接受
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 多重特征约束
pub fn notify_1(item: &(impl Summary + Display)) {}

//where 约束
fn some_function<T, U>(t: &T, u: &U)
    where T: Display + Clone,
          U: Clone + Debug
{}

// 返回实现了Summary特征的类型
fn returns_summarizable() -> impl Summary {
    Weibo {
        username: String::from("sunface"),
        content: String::from(
            "m1 max太厉害了，电脑再也不会卡",
        )
    }
}

#[derive(Debug)]
struct Point<T: Add<T, Output = T>> {
    x: T,
    y: T,
}

impl <T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;
    fn add(self, p: Point<T>) -> Self::Output {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    let post = Post{title: "Rust语言简介".to_string(),author: "Sunface".to_string(), content: "Rust棒极了!".to_string()};
    let weibo = Weibo{username: "sunface".to_string(),content: "好像微博没Tweet好用".to_string()};

    println!("{}",post.summarize());
    println!("{}",weibo.summarize());

    let p1 = Point{x: 1.1f32, y: 1.1f32};
    let p2 = Point{x: 2.1f32, y: 2.1f32};
    println!("{:?}", add(p1, p2));

    let p3 = Point{x: 1i32, y: 1i32};
    let p4 = Point{x: 2i32, y: 2i32};
    println!("{:?}", add(p3, p4));
}
