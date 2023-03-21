fn variables() {
    // By default, all variables in rust are inmutable
    // This ensure:
    //  1. Security, so other parts of the code can modify the value
    //  2. Readability, developers can know when a var will change or no
    let mut x: u8 = 5;
    println!("value of x {x}");
    x = 6;
    println!("value of x {x}");
}

fn constants() {
    // constants are values that are bound to a name and are not allowed to change
    // Even, you are not allowed to use the mut keyword with constants
    // Type annotation it's mandatory for constants
    // constants may be set only to a constant expression, not the result of a value that could only be computed at runtime
    const PI: f64 = 3.14; // Gonna make haters!

    println!("The area of a circle with radius 2cm is {}", PI * f64::powi(2.0, 2));

    // Operations we can do over const at compiling time
    // https://doc.rust-lang.org/reference/const_eval.html
}

fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn main() {
    variables();
    constants();
    shadowing();
}
