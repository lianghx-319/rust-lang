use std::io;

fn main() {
    println!("Guess Number Game!");

    println!("Please guess a number: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Invalid line content!");

    println!("Number: {} is what you guessed", guess.trim());
}
