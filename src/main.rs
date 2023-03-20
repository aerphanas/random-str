use rand::Rng;
use std::{char::from_u32, io::{self, Write}};

fn main() {

    let mut nums: u32 = 0;
    let mut input: String = String::new();

    const LOW_ASCII: u8 = 33;
    const UPP_ASCII: u8 = 126;


    print!("length : ");

    io::stdout()
        .flush()
        .unwrap();

    io::stdin()
        .read_line(&mut input)
        .unwrap();


    if let Ok(num) = input.trim().parse::<u32>() {
        nums = num - 1
    } else {
        println!("error is input number ?")
    }

    (0..=nums).into_iter()
        .for_each(|_| {
            print!("{}", from_u32(rand::thread_rng().gen_range(LOW_ASCII..=UPP_ASCII).into()).unwrap())
        });

    println!("");

}