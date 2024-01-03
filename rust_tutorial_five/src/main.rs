// if and else statements
#![allow(unused)]

// use rand::Rng;
use std::cmp::Ordering;
use std::f64::consts::PI;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::ops::RangeBounds;

// instead of (age >=1) && (age <= 18)
// do (1..=18).contains(age) since the range is inclusive

fn main() {
    let age: i32 = 19;
    if (1..=18).contains(&age) {
        println!("Very Important birthday");
    } else if (age == 21) || (age == 50) {
        println!("Modestly Important birthday");
    } else if (age >= 65) {
        println!("Important birthday");
    } else {
        println!("Not Important birthday");
    }

    // some sort of ternary operator simulation
    // can be reduce to     let can_vote: bool = (my_age >= 18);

    let mut my_age: i32 = 47;
    // streamlined
    let can_vote: bool = (my_age >= 18);
    // let can_vote: bool = if my_age >= 18 {
    //     true
    // } else {
    //     false;
    // };
    // println!("Can Vote");
}
