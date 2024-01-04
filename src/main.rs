#![allow(unused)]

use std::cmp::Ordering;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::num::ParseIntError;

use rand::Rng;
fn main() {
    //casting

    // let num: u8 = 9;
    // let num2: u8 = 99;
    // let num3: u32 = (num as u32) + (num2 as u32);

    // let str = "sand";

    // let st: String = String::from("hola");

    // println!("{}", add);

    //ENUMS
    enum Day {
        Sun,
        Mon,
        Tue,
        Wed,
        Thu,
        Fri,
        Sat,
    }

    fn todo(data: &str) -> String {
        let mut st = String::from( "hola nice ");
        st.push_str(data);
        st.push_str("today");
        return st;
    }

    impl Day {
        fn guess_me(&self) -> String {
            match self {
                Day::Sun => todo("weekend"),
                Day::Mon => todo("Monday"),
                Day::Tue => todo("Tuesday"),
                Day::Wed => todo("Wednesday"),
                Day::Thu => todo("Thursday"),
                Day::Fri => todo("Friday"),
                Day::Sat => todo("weekend"),
                _ => todo("de nada"),
            }
        }
    }

    
    //vectors
    let vec:Vec<i32> = Vec::new();
    let mut vec2 = vec![1,2,3,4];
    vec2.push(23);
    for v in &vec2{
        print!("{},", v)
    }
    for i in &mut vec2{
        *i *=2 ;
    }
    println!("");
    for v in &vec2{
        print!("{},", v);
    }



}
