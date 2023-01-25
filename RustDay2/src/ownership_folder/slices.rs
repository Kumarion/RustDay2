pub fn slices () {
    // Slices are a reference to a part of a string
    // They are immutable by default
    // They are a pointer to a part of a string
    // They are a pointer to a part of an array
    // They are a pointer to a part of a vector
    // Do not take ownership of the underlying data

    // String slices
    let mut s = String::from("Hello world");
    let s2 = "hello world";
    let word = first_word(s2);
    println!("The first word is: {}", word);

    // String slices with mutable references
    let mut s3 = String::from("Hello world");
    let word2 = first_word(&s3);
    println!("The first word is: {}", word2);
    
    // Slices with array
    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..2];
    println!("The slice is: {:?}", slice);
}

fn first_word (s: &str) -> &str {
    // Convert string to bytes
    let bytes = s.as_bytes();

    // Iterate over the bytes
    for (i, &item) in bytes.iter().enumerate() {
        // If the byte is a space, return the index
        if item == b' ' {
            return &s[0..i];
        }
    }

    // If no space is found, return the length of the string
    &s[..]
}