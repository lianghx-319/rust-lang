use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess Number Game!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Secret Number is: {}", secret_number);

    println!("Please guess a number: ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Invalid line content!");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("Number You Guess is: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("To big!"),
        Ordering::Equal => println!("Bingo!"),
    }
}
