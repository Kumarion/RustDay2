
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello");
    some_string
}

pub fn ownership() {
    let x = 5;
    let y = x; // x is moved to y
    // x is no longer valid, y is the only valid reference to the value

    let s1 = String::from("Hello");
    let s2 = s1; // s1 is moved to s2
    // s1 is no longer valid, s2 is the only valid reference to the string

    // Here's an example of cloning, instead of moving
    let s3 = String::from("Hello");
    let s4 = s3.clone(); // s3 is copied to s4
    // s3 is still valid, s4 is a copy of s3

    // Ownership and functions
    let s = String::from("Hello");
    takes_ownership(s); // s is moved to takes_ownership
    // s is no longer valid

    // Ownership and return values
    let s5 = gives_ownership(); // s5 is moved to gives_ownership
    // s5 is valid

    // Ownership and scope
    let s6 = String::from("Hello");
    {
        let s7 = String::from("Hello");
        // s7 is valid
    } // s7 is no longer valid

    // Use variable without taking ownership
    let s8 = String::from("Hello");
    let s9 = &s8;
    // s8 is still valid, s9 is a reference to s8
    // Get more into that in references.rs
}