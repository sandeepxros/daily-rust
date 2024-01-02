#![allow(unused)]

use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::num::ParseIntError;
fn main() {
    //type string
    const MYNAME: &str = "sandeep";
    //type number
    const MY_NUM: u32 = 1_000_000;

    const PI: f32 = 3.141592;

    let myage = "28232";

    let mut myage: u32 = myage.trim().parse().expect("sorry can't convert it");
    myage = myage + 1;
    println!("{} and myage  {} ", MYNAME, myage);


    println!("Unsigned Interget u32 Max: {}, Min : {}", u32::MAX, u32::MIN);

    println!("signed Interget i32 Max: {}, Min : {}", i32::MAX, i32::MIN);

    println!("max usize {} , min: {}", usize::MAX, usize::MIN);

    let is_true = true;
    let _istrue = true; // unused variable if you want to give

    let my_grade = 'A';

    



}
