enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:#?}", &self);
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    let m = Message::Write(String::from("hello"));
    m.call();

}

// the Option type
enum Option<T> {
    Some(T),
    None,
}

fn option_examples() {
    let some_number = Some(5);
    let some_string = Some("a string");

    // If we use None rather than Some, we need to tell Rust what type of Option<T> we have, because the compiler can’t infer the type
    // let absent_number: Option<i32> = None;
}

// match operator
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// come back to if let when you're not so tired