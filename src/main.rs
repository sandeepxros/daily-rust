#![allow(unused)]

use std::cmp::Ordering;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::num::ParseIntError;

use rand::Rng;
fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut index = 0;
    let mut arr2: [i32; 9] =[0,0,0,0,0,0,0,0,0];

    // loop { 
    //     if index == arr.len() {
    //         println!("Quitting...");
    //         break;
    //     }  
    //     if arr[index] % 2 == 0 {
    //         println!("even number found: {} ", arr[index]);
    //         index+=1;
    //         continue;
    //     }
    //     else {
    //         println!("odd number found: {} ", arr[index]);
    //         index+=1;
    //         continue;
    //     }
    // }

    // while index<arr.len() {
    //     println!("value : {}", arr[index]);
    //     index+=1;
    // }

    // for val in arr.iter(){
    //     println!("value : {}", val);
    // }

    //tuple 
    let my_tuple:(i32, String, f32, i32)  = (32, "SAndy".to_string(), 3.2, 2);
    println!("Value in my tuple :{}", my_tuple.1);

    let (v1, v2, v3, v4) = my_tuple;
    println!("tuple values {} {} {} {}", v1,v2, v3, v4);

}
