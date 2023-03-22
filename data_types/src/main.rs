use std::io::stdin;

fn scalar_types() {
    // A scalar type represents a single value. Rust has four primary scalar
    // types: integers, floating-point numbers, Booleans, and characters

    // Integer types
    // Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
    // Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
    // The default integer type is i32. This is generally the fastest
    println!("Integer types:");
    let x: i32 = 1;
    println!("x = {x}"); // -> x = 1

    // Integer literals can be type annotated by adding the type as a suffix
    // or by using a _ as a visual separator. For example, 98_222 is the same

    let x: u32 = 1_000;
    println!("now x = {x}"); // -> now x = 1000

    // Integers also can be written in decimal (98_222), hexadecimal (0xff),
    // octal (0o77), or binary (0b1111_0000). The type of the integer is
    // determined by the compiler based on the number used.
    let x: u32 = 0xff;
    println!("now x = {x}"); // -> now x = 255

    // Integer overflow
    // When a number exceeds the maximum value for its type, it “wraps around”
    // to the minimum value for the type. This behavior is intended and is
    // part of Rust’s core safety guarantees

    /*
    let x: u8 = 256;
    println!("now x = {x}"); // -> x = 0
    let x: u8 = 257;
    println!("now x = {x}"); // -> x = 1
    */  // -> This code will not compile

    // Floating-point types
    // Rust has two primitive types for floating-point numbers: f32 and f64.
    // The default type is f64 because on modern CPUs it’s roughly the same
    // speed as f32 but is capable of more precision.
    // f32 is a single-precision float, and f64 has double precision.
    println!("Floating-point types:");
    let x: f32 = 2.3;
    println!("x = {x}"); // -> x = 2.3

    // Numeric operations
    println!("Numeric operations:");
    calculator();

    // Boolean type
    // A boolean type has two possible values: true and false.
    println!("Boolean type:");
    let x: bool = true;
    println!("x = {x}"); // -> x = true

    // The char type
    // The char type is four bytes in size and represents a Unicode Scalar
    // Value, which means it can represent a lot more than just ASCII.
    // Accents and Chinese/Japanese/Korean characters are all valid char
    // values.
    println!("The char type:");
    let x: char = 'x';
    println!("x = {x}"); // -> x = x
    let x: char = '😻';
    println!("x = {x}"); // -> x = 😻
}

// Practicing with numeric operations
fn calculator() {
    fn add(x: i64, y: i64) -> i64 {
        x + y
    }

    fn sub(x: i64, y: i64) -> i64 {
        x - y
    }

    fn mul(x: i64, y: i64) -> i64 {
        x * y
    }

    fn div(x: i64, y: i64) -> f64 {
        x as f64 / y as f64
    }

    fn rem(x: i64, y: i64) -> i64 {
        x % y
    }

    // Request two numbers to the user
    loop {
        let mut x: String = String::new();
        let mut y: String = String::new();

        println!("Enter the first number: ");

        stdin().read_line(&mut x).expect("Failed to read line");

        let x: i64 = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Enter the second number: ");

        stdin().read_line(&mut y).expect("Failed to read line");

        let y: i64 = match y.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{} + {} = {}", x, y, add(x, y));
        println!("{} - {} = {}", x, y, sub(x, y));
        println!("{} * {} = {}", x, y, mul(x, y));
        println!("{} / {} = {}", x, y, div(x, y));
        println!("{} % {} = {}", x, y, rem(x, y));

        println!("Do you want to continue? (y/n)");
        let mut answer: String = String::new();
        stdin().read_line(&mut answer).expect("Failed to read line");

        if answer.trim() == "n" {
            break;
        }
    }
}

fn compound_types() {
    // Compound types can group multiple values into one type. Rust has two
    // primitive compound types: tuples and arrays.

    // Tuples have a fixed length: once declared, they cannot grow or shrink
    // in size.
    // Each position in the tuple has a type, and the types of different
    // positions don’t have to be the same.
    // Tuples are useful for returning multiple values from a function
    // without having to create a custom struct to carry all the values.
    println!("Tuples:");
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructuring a tuple
    // We can use pattern matching to destructure a tuple value.
    // This breaks the single tuple into three parts and binds each part to a
    // variable.
    let (x, y, z) = tup;
    println!("x = {x}, y = {y}, z = {z}"); // -> x = 500, y = 6.4, z = 1

    // We can also access a tuple element directly by using a period (.) followed
    // by the index of the value we want to access.
    let x: i32 = tup.0;
    let y: f64 = tup.1;
    let z: u8 = tup.2;
    println!("x = {x}, y = {y}, z = {z}"); // -> x = 500, y = 6.4, z = 1

    // Arrays have a fixed length, like tuples.
    // Arrays in Rust are different from arrays in some other languages because
    // arrays in Rust have a fixed length, like tuples.
    // Arrays are useful when you want your data allocated on the stack rather
    // than the heap.
    // All the elements of an array must have the same type.
    println!("Arrays:");
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a = {:?}", a); // -> a = [1, 2, 3, 4, 5]

    // We can also initialize all the elements of an array to the same value
    // if we specify the initial value, followed by a semicolon, and then the
    // length of the array in square brackets.
    let a: [i32; 5] = [3; 5];
    println!("a = {:?}", a); // -> a = [3, 3, 3, 3, 3]
                             //
    // Accessing array elements
    // We can access an element in an array by indexing the array.
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a[0] = {}", a[0]); // -> a[0] = 3

    // We can also use the get method with the index passed as an argument to
    // get the value at that index.
    println!("a.get(0) = {:?}", a.get(0)); // -> a.get(0) = Some(3)

    // We can also access a range of elements in an array using the range
    // syntax.
    println!("a[0..2] = {:?}", &a[0..2]); // -> a[0..2] = [3, 3]
}

fn main() {
    scalar_types();
    compound_types();
}
