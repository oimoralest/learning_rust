use crate::utils::_char;
/// This function checks if a string contains only letters
///
/// # Arguments:
///     * s - The string to check.
///
/// # Return: true if the string contains only letters, false otherwise.
pub fn is_only_letters(s: &str) -> bool {
    s.chars().all(|c| _char::is_lowercase(c) || _char::is_uppercase(c))
}
