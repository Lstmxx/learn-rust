struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle { x, y, radius }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

/**
 * 用pub的方式实现getter，别的文件就不能直接访问width了
 */
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height 
    }
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    pub fn width(&self) -> u32 {
        return self.width
    }
}

fn main() {
    let circle = Circle::new(1f64, 2f64, 3f64);
    println!("circle area is {}", circle.area());
    let rectangle = Rectangle::new(12u32, 12u32);
    println!("rectangle area is {}", rectangle.area());
    println!("width: {}", rectangle.width());
}
