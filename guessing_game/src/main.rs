use std::cmp::Ordering;
use std::io;
use std::process::exit;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut my_guess = String::new();

        io::stdin().read_line(&mut my_guess)
            .expect("Failed to read line");

        let my_guess: i32 = match my_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            },
        };

        match my_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

}
