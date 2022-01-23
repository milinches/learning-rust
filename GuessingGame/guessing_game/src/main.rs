// extern allows rust to know we are trying to use the package.
extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("A guessing Game\n");

    println!("Guess the number\n");

    loop {
        let secret_number: u64 = rand::thread_rng().gen_range(1..101);
        println!("The secret number is {}", secret_number);

        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u64 = guess.trim().parse()
            .expect("Please use a number");
        
        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Greater => println!("Too large!!"),
            Ordering::Equal => {
                println!("You won");
                break;
            }, 
        }
    }
}