#![allow(unused)]
// References just makes referneces to variables
// If you want to use a variable WITHOUT taking ownership, use references

fn calculate_length(s: &String) -> usize {
    s.len()
}

// Here's an example of mutable references
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

pub fn references() {
    // References are immutable by default
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // Heres an example of mutable references
    let mut s2 = String::from("Hello");
    change(&mut s2);
    println!("{}", s2);

    // You can only have one mutable reference to a variable at a time
}