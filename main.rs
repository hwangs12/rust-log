fn main() {
    println!("Hello, world!");

    // tuple 
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // destructuring
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
}