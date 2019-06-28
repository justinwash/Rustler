fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.
  
fn _mutable_reference() {
    let mut s = String::from("hello");
    _change(&mut s);
}

fn _change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn _reference_restrictions() {
    let mut s = String::from("hello");
    
    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    
    let _r2 = &mut s;
}