// standard input/output library for rust
use std::io;
// using the rand crate
// Rng trait defines methods for random number generation
use rand::Rng;
// Ordering type is anenum with variants Less, Greater, Equal
// cmp method compares two values, takes an argument to compare with
use std::cmp::Ordering;

    // thread_rng method is a random number generator
    // which is local to the current thread of execution
    // gen_range method takes a range expression as an argument
    // and generates a random number within that range
    // this range takes the form of start.. =end

    // calling cmp (compare) on guess returns a variant of Ordering
    // this may be Greater, Equal or Less
    // the match expression to decide what to do based on the return

fn main() {
    println!("Welcome to Guess The Number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!!!");
                break;
            },
        }
    }
}
