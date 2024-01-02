#![allow(unused)]

use std::cmp::Ordering;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::num::ParseIntError;

use rand::Rng;
fn main() {
    
    // maths operation today we are going to read

    //decimal percision
    let num_1: f32 = 1.1111111111;
    let num_2: f64 = 1.1111111111;
    println!("f32: {} ", num_1 + 0.1111111111);
    println!("f64: {} ", num_2 + 0.1111111111);

    //arithmatic operations;

    let mut num3: i32 = 12;
    let num4: i32 = 5;

    println!("12 + 5 = {}", num3 + num4);
    println!("12 - 5 = {}", num3 - num4);
    println!("12 * 5 = {}", num3 * num4);
    println!("12 / 5 = {}", num3 / num4);
    println!("12 % 5 = {}", num3 % num4);
    num3 += 1;
    println!("num3+= : {}", num3);
    num3 *= 2;
    println!("num3*= : {}", num3);
    num3 /= 2;
    println!("num3/= : {}", num3);
    num3 -= 5;
    println!("num3-= : {}", num3);
    num3 %= 5;
    println!("num3%= : {}", num3);

    //random number
    let rand = rand::thread_rng().gen_range(1..101);
    println!("Generated random number: {} ", rand);

    

    //conditionals
    let mut age = -101;

    /*
    if (age > 16) && (age < 14) {
        println!("Congrats you can come into party.");
    } else if (age > 19) || (age > 17) {
        println!("please don't come without gifts");
    } else if age == 12 {
        println!("sorry you can't enter");
    } else {
        println!("go home buddy");
    }
     */

    //ternary equivalent
    /*
    let mut is_Adult:bool = if age>=18 {
        true
    }else {
        false
    };
    println!(" is it an adult : {}", is_Adult)
    */

    /*
    //match statement

    match age {
        1..=17 => println!("You are bw 1,18"),
        18 | 20 => println!("you are an adult"),
        18..=50 => println!("you are a parent"),
        18..=i32::MAX => println!("I dont know"),
        _ => println!("whatever"),
    }
    */

    // match with cmp and ordering
    let voting_age = 18;
    match age.cmp(&voting_age) {
        Ordering::Less => println!("you can't vote"),
        Ordering::Greater => println!("you can vote"),
        Ordering::Equal => println!("you can start voting"),
    }
}
