fn _example() {         // s is not valid here, it’s not yet declared
    let s = "hello";    // s is valid from this point forward
    let _d = s;         // do stuff with s
}                       // this scope is now over, and s and d are no longer valid

fn main() {
    // String is different from a "string literal" cause can be mutable
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`
}

fn _move() {
    // value type (stored on the stack)
    let x = 5;
    let _y = x; // *copy* x and assign the value to y *on the stack*
    
    // reference/pointer type (stored on the heap)
    let s1 = String::from("hello");
    let _s2 = s1; // copy the *pointer to data stored on the heap* from s1 to s2
    
    // println!("{}, world!", s1); would throw an error because s1 is no longer valid
    
    let s1 = String::from("hello");
    let _s2 = s1.clone(); // clone (deep copy) s1 to s2, allocating memory for both

    println!("s1 = {}, s2 = {}", s1, _s2);
}

fn _functions_and_ownership() {
    let s = String::from("hello");  // s comes into scope
    _takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    let x = 5;                      // x comes into scope
    _makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn _takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn _makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn _return_types_and_owndership() {
    let _s1 = _gives_ownership();         // gives_ownership moves its return
                                        // value into s1
    let s2 = String::from("hello");     // s2 comes into scope
    let _s3 = _takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.
  
fn _gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it
    let some_string = String::from("hello"); // some_string comes into scope
    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn _takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
} 
