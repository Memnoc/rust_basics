// arrays and loops
// elements must be the same data type
// array must have fixed size
#![allow(unused)]
use std::cmp::Ordering;

fn main() {
    let arr_1 = [1, 2, 3, 4, 5, 7, 8, 9];
    println!("1st : {}", arr_1[0]);
    println!("Length : {}", arr_1.len());
    // let's loop
    let mut loop_index: usize = 0;
    loop {
        // skip the even values
        if arr_1[loop_index] % 2 == 0 {
            loop_index += 1;
            continue;
        }
        if arr_1[loop_index] == 9 {
            break;
        }
        println!("Arr loop : {}", arr_1[loop_index]);
        loop_index += 1;
    }
    // while loop
    while loop_index < arr_1.len() {
        println!("Arr while loop : {}", arr_1[loop_index]);
        loop_index += 1;
    }
    // for loops
    for val in arr_1.iter() {
        println!("Arr for loop : {}", val);
    }
}
