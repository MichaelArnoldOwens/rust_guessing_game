use rand::Rng;
use std::cmp::Ordering;
use std::io; // standard library prelude https://doc.rust-lang.org/std/prelude/index.html

fn main() {
    // entry point
    println!("Guess the number!"); // println! is a macro

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // variables are immutable by default so use 'mut' keyword
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // reading user input, &mut guess is a ref to guess... *returns a io:Result
            // Result is an enum - Ok or Err
            .expect("Failed to read line"); // error handling; without this we get a compile warning saying error should be handled. this will just crash

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess); // string interpolation

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
