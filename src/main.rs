use rand::Rng;
use std::{char::from_u32, io};

fn main() {

    println!("length : ");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut nums: u32 = 0;

    if let Ok(num) = input.trim().parse::<u32>() {
        nums = num - 1
    } else {
        println!("error is input number ?")
    }

    for _ in 0..=nums {
        print!("{}", from_u32(rand::thread_rng().gen_range(33..=126)).unwrap())
    }

}