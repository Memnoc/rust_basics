// generating random values
#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::f64::consts::PI;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    // generate a random number berween 1 and 100 - 101 cause goes up to 100
    let random_num: i32 = rand::thread_rng().gen_range(1..101);
    println!("Random number = {}", random_num);
}
