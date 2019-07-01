fn main() {
    println!("{}", first_word(&String::from("Hello, world!")));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    
    &s
}

fn _slice_from_beginning() {
    let s = String::from("hello");

    let _slice = &s[0..2];
    let _slice = &s[..2];
}

fn _examples() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

fn _slicing_arrays() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}