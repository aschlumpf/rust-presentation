use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    let mut guess = String::new();
    loop {
        guess.clear();
        println!("Please input your guess.");
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Guess must be a number.");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too large!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    


    // println!("You guessed: {}", guess);
}