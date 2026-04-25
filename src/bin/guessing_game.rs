use rand::prelude::*;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number");
    let mut rng = rand::rng();
    // can do done also like rand::random_range(1..=100);
    let secret_number: u32 = rng.random_range(1..=100); // in rust book it is rand::thread_rng().gen_range(1..=100) which is in some old version of rand

    loop {
        println!("Please input the number you guess: ");

        let mut guess = String::new();

        // to get user input and send it to guess
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            // check is mut guess which is parse is of type u32, if yes assign the value to the var and if not, continue
            Ok(some) => some, // here some is a comparison place holder, some can be any or num or any name you want
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
