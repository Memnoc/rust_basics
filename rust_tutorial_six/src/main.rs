// match statement
#![allow(unused)]
use std::cmp::Ordering;

fn main() {
    let age: i32 = 119;
    match age {
        1..=18 => println!("Important Birthday"),
        19..=100 => println!("You are already old!"),
        _ => println!("Not an important birthday"), // the _ matches every other case
    };

    // now let's use some Ordering functions
    let my_age: i32 = 18;
    let voting_age: i32 = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("You cannot vote yet"),
        Ordering::Equal => println!("Enjoy your first voting experience"),
        Ordering::Greater => println!("You can also vote"),
    };
}
