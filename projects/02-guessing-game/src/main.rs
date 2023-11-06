use core::cmp::Ordering;
use rand::Rng;
use std::io;

fn random_number_1_to_10() -> u16 {
    return rand::thread_rng().gen_range(1..=10);
}

fn main() {
    println!("Guess the number!");

    let correct_answer = random_number_1_to_10();
    let mut guess: String;

    loop {
        guess = String::new();
        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess_uint: u16 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {guess}");

        match guess_uint.cmp(&correct_answer) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    println!("Your guess {guess} was correct!");
    println!("Done!");
}
