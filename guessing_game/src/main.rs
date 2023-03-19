// std: standard library
// io: input/output library/module
use std::io::stdin;
// cmp: module with functionality and enums to compare variables
use std::cmp::Ordering;
// rand: External/Third-party crate
// Rng: The Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods
use rand::Rng;

// fn: keyword for defining a function
// main: name of the function. This is the entry point of the program.
/**
 * main: Entrypoint
 *
 * Return: na
 * */
fn main() {
    // println! is a macro that prints a string to the console
    println!("Guess the number!");

    // Run cargo doc --open will show us the doc used by the project
    let secret_value: u8 = rand::thread_rng().gen_range(1..=100);

    let mut count: i8 = 1;
    loop {
        // let: keyword for creating a variable
        // mut: keyword for making a variable mutable
        // guess: name of the variable
        // String: type of the variable provided by the std library
        // ::new: associated function of the String type. An associated function is a function that’s implemented on a type, in this case String
        let mut guess: String= String::new();

        println!("Please input your guess.");

        // stdin(): returns an instance of std::io::Stdin
        // read_line(): method of Stdin
        // &: indicates that this argument is a reference. References are inmutable
        // expect: Captures Result’s variants Ok or Err. If Result contains Err, stops the program and print the message
        stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            },
        };

        // {}: Placeholder for variables
        println!("You guessed: {guess}");

        match guess.cmp(&secret_value) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congrats, you guessed!");
                break;
            }
        }

        println!("You still have {} tries", 7 - count);
        count += 1;

        if count == 7 {
            println!("Sorry, you loose...");
        }
    }
}
