fn main() {
    vector_examples();
    string_examples();
}

fn vector_examples() {
    let mut v: Vec<i32> = Vec::new();
    
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    
    println!("{:?}", v);
    
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    
    let does_not_exist = v.get(100); // returns None instead of crashing like &v[100] would
    println!("{:?}", does_not_exist);
    
    for i in &mut v {
        *i += 50;
    }
    
    println!("{:?}", v);
    
    // enums and vectors
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
    println!("{:?}", row);
}

fn string_examples() {
    
}