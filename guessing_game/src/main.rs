// std: standard library
// io: input/output library
use std::io;

// fn: keyword for defining a function
// main: name of the function. This is the entry point of the program.
fn main() {
    // println! is a macro that prints a string to the console
    println!("Guess the number!");

    println!("Please input your guess.");

    // let: keyword for creating a variable
    // mut: keyword for making a variable mutable
    // guess: name of the variable
    // String: type of the variable
    // ::new: associated function of the String type
    let mut guess: String = String::new();

    // &: reference operator
    // &mut guess: mutable reference to guess
    // .read_line: method of the io::stdin() object
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);

}
