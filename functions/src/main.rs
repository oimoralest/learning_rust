mod utils;
use utils::_char;
use utils::_string;
use utils::_number;

fn main() {
    without_parameters();
    say_hello("Rustacean");

    println!("The reminder of 10 / 3 is {}", reminder_of(10, 3));

    println!("Is 'a' a lowercase letter? {}", _char::is_lowercase('a'));
    println!("Is 'A' a lowercase letter? {}", _char::is_lowercase('A'));
    println!("Is 'a' a uppercase letter? {}", _char::is_uppercase('a'));
    println!("Is 'A' a uppercase letter? {}", _char::is_uppercase('A'));
    println!("Is 'Rust' a _string with only letters? {}", _string::is_only_letters("Rust"));
    println!("Is 'Rust 2021' a _string with only letters? {}", _string::is_only_letters("Rust 2021"));
    println!("The last digit of 123456 is {}", _number::last_digit(123_456));
    println!("The last digit of -123456 is {}", _number::last_digit(-123_456));
    println!("The first digit of 123456 is {}", _number::first_digit(123_456));
    println!("The first digit of -123456 is {}", _number::first_digit(-123_456));
}


/// without_parameters: This function print a message without parameters
///
/// # Arguments: None
///
/// # Return: void
fn without_parameters() {
    println!("Hello, world!");
}


/// say_hello: This function print a message with a name
///
/// # Arguments:
///     * name -  The name of the person to greet.
///
/// # Return: void
fn say_hello(name: &str) {
    println!("Hello, {}!", name);
}


/// reminder_of: This function return the reminder of a division
///
/// # Arguments:
///     * x - The dividend.
///     * y - The divisor.
///
/// # Return: The reminder of the division
fn reminder_of(x: i64, y: i64) -> i64 {
    x % y
}
