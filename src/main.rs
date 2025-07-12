#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let mut name: String = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin()
        .read_line(&mut name)
        .expect("Didn't receive Input");
    
    println!("Hello {}! {}", name.trim_end(), greeting);
}

/*
    // === Hello world! === // 
    fn main() {
        let mut name: String = String::new();
        let greeting: &str = "Nice to meet you";
        io::stdin()
            .read_line(&mut name)
            .expect("Didn't receive Input");

        println!("Hello {}! {}", name.trim_end(), greeting);
    }
*/