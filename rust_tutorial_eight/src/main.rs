// tuples
#![allow(unused)]
use std::cmp::Ordering;

fn main() {
    // classic tuple declaration
    let my_tuple: (u8, String, f64) = (47, "Memnoc".to_string(), 50_000.00);
    println!("Name : {} Age : {}", my_tuple.1, my_tuple.2);

    // destructuring
    let (v1, v2, v3) = my_tuple;
    println!("Age : {}", v1);
    println!("Name : {}", v2);
    println!("Number : {}", v3);
}
