use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Number Guessing Game!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    // println!("Generated number:{secret_number}");

    println!("Entet your guess:");

    loop {
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Something went wrong");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(..) => {
                println!("Please Enter a valid number!!!");
                continue;
            }
        };

        println!("Your guess is: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You guessed it right!");
                break;
            }
            Ordering::Less => {
                println!("You guess is less than the secret number ")
            }
            Ordering::Greater => {
                println!("You guess is greater than the secret number ")
            }
        }
    }
}
