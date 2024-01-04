// Strings
fn main() {
    let mut string_one = String::new();
    string_one.push('A');
    string_one.push_str(" word");
    // iterate the word at blank space
    for word in string_one.split_whitespace() {
        println!("{}", word);
    }
    let string_two = string_one.replace('A', "Another");
    println!("{}", string_two);
}
