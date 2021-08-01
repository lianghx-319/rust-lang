use std::io;

use rand::Rng;

fn main() {
    println!("Guess Number Game!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Secret Number is: {}", secret_number);

    println!("Please guess a number: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Invalid line content!");

    println!("Number You Guess is: {}", guess.trim());
}
