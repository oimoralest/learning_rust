mod fahrenheit_celsius;
mod fibonacci;

use std::io::stdin;

use fahrenheit_celsius::convert;
use fibonacci::fibonacci;

fn main() {
    convert();
    get_fibonacci();
}

fn get_fibonacci() {
    let mut input: String = String::new();
    println!("Enter a number: ");
    stdin().read_line(&mut input).expect("Failed to read line");
    let n: u32 = input.trim().parse().expect("Please type a number!");
    println!("Fibonacci of {} is {}", n, fibonacci(n));
}
