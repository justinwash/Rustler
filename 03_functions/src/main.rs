fn main() {
    println!("Hello, world!");
    
    another_function(5);
    println!("{}", return_value());
    println!("{}", plus_one(5));
}

fn another_function(x: i32) {
    let _x = x;
    
    let y = {
        let x = 3;
        x + 1 // no semicolon: return value, semicolon: assignment only
    };

    println!("The value of y is: {}", y);
}

fn return_value() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
