use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::rng().random_range(1..=100);

    println!("Guess the lucky integer!");

    loop {
        println!("Enter your guess:");

        let mut guess_text = String::new();
        io::stdin()
            .read_line(&mut guess_text)
            .expect("Failed to read line");

        let guess: u32 = guess_text.trim().parse().expect("Invalid integer!");
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("Win");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }

        guess_text.clear();
    }
}
