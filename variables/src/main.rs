const MAX_POINTS: u32 = 100_000;

fn main() {
    // const
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    
    // mutability
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    
    // shadowing
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is: {}", spaces);
    
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}, {}", y, tup.1);
    
    // array
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // or just let a = [0; 5]
}