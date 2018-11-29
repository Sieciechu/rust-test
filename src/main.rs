extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    const MIN: i32 = 1;
    const MAX: i32 = 10;
    
    println!("Guess the number from {} to {}.", MIN, MAX);

    let secret_number = rand::thread_rng().gen_range(MIN,MAX+1);
    println!("The secret number is {}\n\n", secret_number);

    loop {
        println!("Input the guess.");
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number");
                continue;
            }
        };
        
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }

        }
    }
}
