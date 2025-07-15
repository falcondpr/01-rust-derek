#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let arra_1: [i32; 6] = [1,2,3,4,5,6];
    // println!("2st: {}", arra_1[1]);
    // print!("Length: {}", arra_1.len());
    let mut loop_idx = 0;
    loop {
        if arra_1[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }

        if arra_1[loop_idx] == 6 {
            break;
        }
        println!("Val: {}", arra_1[loop_idx]);
        loop_idx += 1;
    }
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

/*
    // === Varibles and Mutability === //
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age: &str = "47";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number!");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
*/

/*
    // === Numbers === //

    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);

    let is_true: bool = true;
    let my_grade: char = 'A';

    let num_1: f32 = 1.111111111111111;
    println!("f32: {}", num_1 + 0.111111111111111);

    let num_2: f64 = 1.111111111111111;
    println!("f32: {}", num_2 + 0.111111111111111);

    let num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 % 4 = {}", num_3 % num_4);

    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random: {}", random_num);
*/

/*
    // === If conditional === //
    let age: i32 = 47;
    if (age >= 1) && (age <= 18) {
        println!("Important Birthday");
    } else if (age == 21) || (age == 50) {
        println!("Important Birthday");
    } else if age >= 65 {
        println!("Important Birthday");
    } else {
        println!("Not an Important Birthday");
    }
*/

/*
    // === Ternary and Match === //
    let mut my_age = 47;
    let can_vote = if my_age >= 18 { true } else { false };
    println!("Can vote: {}", can_vote);

    let age2: i32 = 8;
    match age2 {
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthday"),
        65..=i32::MAX => println!("Important Birthday"),
        _ => println!("Not an Important Birthday"),
    }
*/

/*
    // === Comparator and Match === //
    let my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't vote"),
        Ordering::Greater => println!("Can vote"),
        Ordering::Equal => println!("You gained the right to vote")
        _ => println!("Not an Important Birthday"),
        };
*/
