/// This function checks if a character is a lowercase letter
///
/// # Arguments:
///     * c - The character to check.
///
/// # Return: true if the character is a lowercase letter, false otherwise.
pub fn is_lowercase(c: char) -> bool {
    c >= 'a' && c <= 'z'
}


/// This function checks if a character is a uppercase letter
///
/// # Arguments:
///     * c - The character to check.
///
/// # Return: true if the character is a uppercase letter, false otherwise.
pub fn is_uppercase(c: char) -> bool {
    c >= 'A' && c <= 'Z'
}
