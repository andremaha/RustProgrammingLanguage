use std::io;
use rand::prelude::*;
use std::cmp::Ordering;

fn main() {

    // Generate a number between 1 and 100
    let number = thread_rng().gen_range(1..101);

    println!("I have a number in my memory between 1 and 100.");

    println!("Can you guess it?");

    loop {
        let mut guess = String::new();

        // Read the user input
        io::stdin().read_line(&mut guess)
            .expect("Could not read your input, mate!");

        // Convert to number
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Are you sure that was number?");
                continue;
            }
        };

        // Let's comare the numbers
        match number.cmp(&guess) {
            Ordering::Less => println!("Nope, the number is smaller!"),
            Ordering::Greater => println!("Nope, the number is greater!"),
            Ordering::Equal => {
                println!("Yep!");
                break;
            }
        }
    }

}   
