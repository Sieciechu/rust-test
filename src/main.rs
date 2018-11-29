use std::io;

fn main() {
    println!("Guess the number");

    println!("Input the guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    let x = 1;
    let y = 6;
    println!("x={:?}, y={}, x+y={}", guess, y, x+y );
    
}