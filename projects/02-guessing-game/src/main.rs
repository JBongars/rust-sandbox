use rand::Rng;
use std::io;

fn random_number_1_to_10() -> u16 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(1..=10);
}

fn main() {
    println!("Guess the number!");

    let correct_answer = random_number_1_to_10();
    let mut is_correct: bool = false;
    let mut guess: String;

    loop {
        guess = String::new();
        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();

        println!("You guessed: {guess}");

        match guess.parse::<u16>() {
            Ok(num) => {
                if num != correct_answer {
                    println!("Guess is incorrect! Try again...");
                } else {
                    is_correct = true;
                }
            }
            Err(e) => {
                println!("Invalid entry");
                println!("error: {e}");
            }
        }

        if is_correct {
            break;
        }
    }

    println!("Your guess {guess} was correct!");
    println!("Done!");
}
