use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // Accept user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the user input to unsigned 32-bit integer if the input is a number
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // Compare user input with secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }
}
