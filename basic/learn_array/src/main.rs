fn main() {
    let array: [String; 8] = core::array::from_fn(|_i| String::from("rust is good!"));
    println!("{:?}", array);
    
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let slice: &[i32] = &a[1..3];

    println!("{:?}", slice);
}
