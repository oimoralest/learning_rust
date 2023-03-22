/// This function compute the last digit of a number
///
/// # Arguments:
///     * n - The number to check.
///
/// # Return: last digit of a number
pub fn last_digit(n: i64) -> i64 {
    let mut n = n;
    n %= 10;
    if n < 0 {
        n *= -1;
    }
    n
}


/// This function compute the first digit of a number
///
/// # Arguments:
///     * n - The number to check.
///
/// # Return: first digit of a number
pub fn first_digit(n: i64) -> i64 {
    let mut n = n;
    if n < 0 {
        n *= -1;
    }
    while n >= 10 {
        n /= 10;
    }
    n
}
