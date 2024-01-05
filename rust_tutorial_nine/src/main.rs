// Strings
// IMPORTANT: you can have static(literals) or mutable strings
// STATIC are typically string literals: let hello: &str = "Hello";
// must be known at compile time, cannot change at runtime
// MUTABLE: are the String type let hello: String = "Hello";
// you can use stuff like push_str() on these etc.
// these are heap allocated
#![allow(unused)]

fn main() {
    let mut string_one = String::new();
    string_one.push('A');
    string_one.push_str(" word");
    // iterate the word at blank space
    for word in string_one.split_whitespace() {
        println!("random word : {}", word);
    }
    let string_two = string_one.replace('A', "Another");
    println!("string two : {}", string_two);

    let string_three = String::from("x r t h b k k a m c");
    // store the list in a vector
    let mut v1: Vec<char> = string_three.chars().collect();
    v1.sort(); // sort
    v1.dedup(); // remove duplicates
                // looping characters
    for char in v1 {
        println!("char : {}", char);
    }
    // creating a string literal
    let string_four: &str = "Random string";
    // creating a mutable string
    let mut string_five: String = string_four.to_string();
    println!("string five : {}", string_five);
    // converting a string to an array of bytes
    let byte_array_one = string_five.as_bytes();
    // get a slice of a string
    let string_six = &string_five[0..6];
    // get string length
    println!("String length : {}", string_six.len());
    // raw string literal
    let string_seven = r#"This is a raw string with escape sequences like \n and \t."#;
    println!("string seven : {}", string_seven);
    // delete values in string (mutable)
    println!("{}", string_five.contains("Ra"));
    println!("string five before deletion : {}", string_five);
    string_five.clear();
    println!("string five after deletion : {}", string_five);
    // combine strings
    let string_eight: String = String::from("Just some ");
    let string_nine = String::from("text");
    let string_ten = string_eight + &string_nine;
    // combined words
    println!("string ten : {}", string_ten);
    // printing unicode char for each char in string
    for char in string_ten.bytes() {
        println!("{},", char);
    }
    let v2: Vec<char> = string_ten.chars().collect();
    for letter in v2 {
        println!("{}", letter);
    }
}
