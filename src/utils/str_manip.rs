/// Extracts the first digit in the given string.
pub fn extract_first_digit(text: &str) -> u32 {
    text.chars()
        .find(char::is_ascii_digit)
        .and_then(|ch| ch.to_digit(10))
        .unwrap_or(0)
}

/// Extracts the last digit in the given string.
pub fn extract_last_digit(text: &str) -> u32 {
    text.chars()
        .rfind(char::is_ascii_digit)
        .and_then(|ch| ch.to_digit(10))
        .unwrap_or(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_extracts_first_digits() {
        assert_eq!(extract_first_digit("1abc2"), 1);
        assert_eq!(extract_first_digit("pqr3stu8vwx"), 3);
        assert_eq!(extract_first_digit("a1b2c3d4e5f"), 1);
        assert_eq!(extract_first_digit("treb7uchet"), 7);
    }

    #[test]
    fn it_extracts_last_digits() {
        assert_eq!(extract_last_digit("1abc2"), 2);
        assert_eq!(extract_last_digit("pqr3stu8vwx"), 8);
        assert_eq!(extract_last_digit("a1b2c3d4e5f"), 5);
        assert_eq!(extract_last_digit("treb7uchet"), 7);
    }
}
