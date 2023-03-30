use std::io::stdin;

pub fn faherenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}


pub fn celcius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

pub fn convert() {
    loop {
         let mut input: String = String::new();

        println!("Enter a choice: ");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");

        stdin().read_line(&mut input).expect("Failed to read line");

        let choice: i32 = input.trim().parse().expect("Please type a number!");

        match choice {
            1 => {
                println!("Enter a temperature in Fahrenheit: ");
                let mut input: String = String::new();
                stdin().read_line(&mut input).expect("Failed to read line");
                let f: f64 = input.trim().parse().expect("Please type a number!");
                println!("{}°F is {}°C", f, faherenheit_to_celsius(f));
            },
            2 => {
                println!("Enter a temperature in Celsius: ");
                let mut input: String = String::new();
                stdin().read_line(&mut input).expect("Failed to read line");
                let c: f64 = input.trim().parse().expect("Please type a number!");
                println!("{}°C is {}°F", c, celcius_to_fahrenheit(c));
            },
            _ => {
                println!("Invalid choice!");
                continue;
            }
        }

        println!("Do you want to continue? (y/n)");
        let mut input: String = String::new();
        stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "y" => continue,
            "n" => break,
            _ => {
                println!("Invalid choice!");
                continue;
            }
        }
    }
   }
