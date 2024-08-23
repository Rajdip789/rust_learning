use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("!!! GUESSING GAME !!!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    println!("Input your guess (1 to 100): ");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter valid input: ");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You Won!");
                break;
            }
            Ordering::Greater => println!("Too high!"),
            Ordering::Less => println!("Too low!"),
        }
        println!("Please guess again: ");
    }
}
