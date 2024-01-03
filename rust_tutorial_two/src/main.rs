// variables
#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::f64::consts::PI;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    const ONE_MIL: u32 = 1_000_000;
    let age: &str = "47";
    let mut age: u32 = age.trim().parse().expect("Age was not assigned a number"); // shadowing
    age += 1;
    println!("I am {} and I want ${}", age, ONE_MIL);
    println!("The value of PI is ${}", PI);
}
