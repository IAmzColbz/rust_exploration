use std::io;
use rand::Rng;

fn main() {
    println!("What is your name");

    let mut name: String = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("There was an Err reading line... exiting");

    let name = name.trim();

    let rand_age: u32 = rand::thread_rng().gen_range(1..=75);

    println!("Hello {name}, you are {rand_age} years old.");
}
