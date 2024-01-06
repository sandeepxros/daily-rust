#![allow(unused)]

use std::collections::HashMap;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn sum(list:&[i32])->i32{
    let mut val = 0; 
    for item in list.iter(){
        val+=item; 
    }
    val
}

fn check(val:i32) -> i32{
    val+22
}


fn main() {
    //generics 

    let mut movies_and_hero = HashMap::new();
    movies_and_hero.insert("Animal", "Ranbir");
    movies_and_hero.insert("Robot", "RajniKanth");
    movies_and_hero.insert("Iron Man", "RDJ");

    for(k, v) in movies_and_hero.iter(){
        println!("{} movie has {}", k, v);
    }

    println!("{}", movies_and_hero.get(&&"Animal"))
}
