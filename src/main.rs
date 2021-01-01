use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn read_line(prompt: &str) -> String {
    print!("{} ", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input
}

fn main() {
    println!("Rust guessing game!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Your secret number: {}.", secret_number);

    loop {
        let guess: isize = match read_line("Enter the number:").trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You must provide a valid number.");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("Right guess, you won! :)");
                break;
            }
        };
    }
}
