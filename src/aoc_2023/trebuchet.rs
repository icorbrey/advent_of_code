//! # Day 1: Trebuchet?!
//!
//! In this problem, a file consisting of words separated by newlines must be
//! processed. For example, consider the below file:
//!
//! ```
//! 1abc2
//! pqr3stu8vwx
//! a1b2c3d4e5f
//! treb7uchet
//! ```
//!
//! The program must turn each of these words into a single two digit number in
//! the format `XY`, where `X` is the first digit in the word and `Y` is the
//! last digit in the word. These values must then be added together to create a
//! single calculation value.

use std::fs;

use inquire::Text;

use crate::utils::{
    infra::Problem,
    str_manip::{extract_first_digit, extract_last_digit},
};

pub struct Trebuchet;

impl Problem for Trebuchet {
    fn id(&self) -> &str {
        "Day 1: Trebuchet?!"
    }

    fn run(&self) -> Result<(), ()> {
        let path = Text::new("Path:").prompt().unwrap();
        let words = fs::read_to_string(path).expect("File not found.");
        println!(
            "Result: {}",
            words
                .split('\n')
                .map(extract_number)
                .into_iter()
                .sum::<u32>()
        );
        Ok(())
    }
}

/// Extracts the two-digit number from the given string.
fn extract_number(text: &str) -> u32 {
    10 * extract_first_digit(text) + extract_last_digit(text)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_extracts_numbers() {
        assert_eq!(extract_number("1abc2"), 12);
        assert_eq!(extract_number("pqr3stu8vwx"), 38);
        assert_eq!(extract_number("a1b2c3d4e5f"), 15);
        assert_eq!(extract_number("treb7uchet"), 77);
    }
}
