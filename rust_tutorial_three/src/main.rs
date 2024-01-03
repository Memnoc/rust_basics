// data types
#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::f64::consts::PI;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    // Unsigned integers : u8, u16, u34, u64, u128, usize;
    // Signed integers: i8, i16, i32, i64, i128, isize
    println!("Max u32 : {}", u32::MAX);
    println!("Max u64 : {}", u64::MAX);
    println!("Max u128 : {}", u128::MAX);
    println!("Max f32 : {}", f32::MAX);
    println!("Max f64 : {}", f64::MAX);
    // booleans
    let is_true: bool = true;
    let my_grade: char = 'A';

    let mut num_1: f32 = 1.111111111111111; // the precision here is excessive -> should use f64
    println!("f32 : {}", num_1 + 0.111111111111111);
    let mut num_2: f64 = 1.111111111111111;
    println!("f32 : {}", num_2 + 0.111111111111111);

    let num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 % 4 = {}", num_3 % num_4);

    // if you want to mutate
    let mut mutable_number: u32 = 10;
    // mutable_number += 1;
    println!("10 plus 1 = {}", {
        mutable_number += 1;
        mutable_number
    });
}
