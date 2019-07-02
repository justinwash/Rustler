fn main() {
    vector_examples();
    string_examples();
    hashmap_examples();
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
    // initialize empty string
    let s = String::new();
    
    // different ways to create strings
    let data ="initial contents";
    let s = data.to_string();
    
    let s = "initial contents".to_string();
    
    let s = String::from("initial contents");
    
    let mut s = String::from("foo");
    s.push_str(" bar");
    
    let mut s = String::from("lo");
    s.push('l');
    
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // "Зд" because each of these letters use 2 bytes
    
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    
    println!("{}", s);
}

fn hashmap_examples() {
    use std::collections::HashMap;
    
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    
    let team_name = String::from("Blue");
    if let score = scores.get(&team_name) {
        println!("{:?}", score)
    }
    
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    scores.entry(String::from("Yellow")).or_insert(50);
    
    println!("{:?}", scores);
    
    let text = "hello world wonderful world";

    // clever!
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}