//! # Day 1: Trebuchet?!
//!
//! ## Part 1
//!
//! Extract a 2-digit "calibration value" from each line in the format `XY`, where `X` is the first
//! numeric digit in the line and `Y` is the last numeric digit in the line. In this problem,
//! "numeric digit" refers to any number 0-9. For example:
//!
//! ```
//! 1abc2       -> 12
//! pqr3stu8vwx -> 38
//! a1b2c3d4e5f -> 15
//! treb7uchet  -> 77
//! ```
//!
//! Find the sum of all of the calibration values. For the previous example, the sum is 142.
//!
//! ## Part 2
//!
//! Perform the same operation as Part 1, but also take into account word digits (i.e. "one" or
//! "seven"). For example:
//!
//! ```
//! two1nine         -> 29
//! eightwothree     -> 83
//! abcone2threexyz  -> 13
//! xtwone3four      -> 24
//! 4nineeightseven2 -> 42
//! zoneight234      -> 14
//! 7pqrstsixteen    -> 76
//! ```

use regex::Regex;

use crate::utils::prelude::*;

pub struct Trebuchet;

impl Problem for Trebuchet {
    fn id(&self) -> &str {
        "Day 1: Trebuchet?!"
    }

    fn part_1(&self, input: &str) {
        let sum: u32 = input.split('\n').map(numeric_calibration_value).sum();
        println!("Sum: {}", sum);
    }

    fn part_2(&self, input: &str) {
        let sum: u32 = input.split('\n').map(word_calibration_value).sum();
        println!("Sum: {}", sum);
    }
}

/// Gets the numeric calculation value of a given line.
fn numeric_calibration_value(line: &str) -> u32 {
    10 * first_numeric_digit(line) + last_numeric_digit(line)
}

/// Extracts the first numeric digit in a given line.
fn first_numeric_digit(line: &str) -> u32 {
    *line
        .chars()
        .filter(char::is_ascii_digit)
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>()
        .get(0)
        .unwrap_or(&0)
}

/// Extracts the last numeric digit in a given line.
fn last_numeric_digit(line: &str) -> u32 {
    *line
        .chars()
        .filter(char::is_ascii_digit)
        .filter_map(|c| c.to_digit(10))
        .rev()
        .collect::<Vec<u32>>()
        .get(0)
        .unwrap_or(&0)
}

/// Gets the word calculation value of a given line.
fn word_calibration_value(line: &str) -> u32 {
    10 * first_word_digit(line) + last_word_digit(line)
}

const MATCH_FIRST_DIGIT: &'static str = r"(\d|zero|one|two|three|four|five|six|seven|eight|nine)";

/// Extracts the first word digit in a given line.
fn first_word_digit(line: &str) -> u32 {
    (Regex::new(MATCH_FIRST_DIGIT).ok())
        .and_then(|re| re.captures(line))
        .and_then(|cap| cap.get(1))
        .and_then(|ch| match ch.as_str() {
            "0" | "zero" => Some(0),
            "1" | "one" => Some(1),
            "2" | "two" => Some(2),
            "3" | "three" => Some(3),
            "4" | "four" => Some(4),
            "5" | "five" => Some(5),
            "6" | "six" => Some(6),
            "7" | "seven" => Some(7),
            "8" | "eight" => Some(8),
            "9" | "nine" => Some(9),
            _ => None,
        })
        .unwrap_or(0)
}

const MATCH_LAST_DIGIT: &'static str = r"(\d|orez|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)";

/// Extracts the last word digit in a given line.
fn last_word_digit(line: &str) -> u32 {
    let line = line.chars().rev().collect::<String>();
    (Regex::new(MATCH_LAST_DIGIT).ok())
        .and_then(|re| re.captures(line.as_str()))
        .and_then(|cap| cap.get(1))
        .and_then(|ch| match ch.as_str() {
            "0" | "orez" => Some(0),
            "1" | "eno" => Some(1),
            "2" | "owt" => Some(2),
            "3" | "eerht" => Some(3),
            "4" | "ruof" => Some(4),
            "5" | "evif" => Some(5),
            "6" | "xis" => Some(6),
            "7" | "neves" => Some(7),
            "8" | "thgie" => Some(8),
            "9" | "enin" => Some(9),
            _ => None,
        })
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_numeric_calibration_values() {
        assert_eq!(numeric_calibration_value("1abc2"), 12);
        assert_eq!(numeric_calibration_value("pqr3stu8vwx"), 38);
        assert_eq!(numeric_calibration_value("a1b2c3d4e5f"), 15);
        assert_eq!(numeric_calibration_value("treb7uchet"), 77);
    }

    #[test]
    fn gets_first_numeric_digits() {
        assert_eq!(first_numeric_digit(""), 0);
        assert_eq!(first_numeric_digit("1"), 1);
        assert_eq!(first_numeric_digit("123"), 1);
        assert_eq!(first_numeric_digit("abc123xyz"), 1);
    }

    #[test]
    fn gets_last_numeric_digits() {
        assert_eq!(last_numeric_digit(""), 0);
        assert_eq!(last_numeric_digit("1"), 1);
        assert_eq!(last_numeric_digit("123"), 3);
        assert_eq!(last_numeric_digit("abc123xyz"), 3);
    }

    #[test]
    fn gets_word_calibration_values() {
        assert_eq!(word_calibration_value("two1nine"), 29);
        assert_eq!(word_calibration_value("eightwothree"), 83);
        assert_eq!(word_calibration_value("abcone2threexyz"), 13);
        assert_eq!(word_calibration_value("xtwone3four"), 24);
        assert_eq!(word_calibration_value("4nineeightseven2"), 42);
        assert_eq!(word_calibration_value("zoneight234"), 14);
        assert_eq!(word_calibration_value("7pqrstsixteen"), 76);
        assert_eq!(word_calibration_value("oneight"), 18);
    }

    #[test]
    fn gets_first_word_digits() {
        assert_eq!(first_word_digit(""), 0);
        assert_eq!(first_word_digit("0"), 0);
        assert_eq!(first_word_digit("zero"), 0);
        assert_eq!(first_word_digit("1"), 1);
        assert_eq!(first_word_digit("one"), 1);
        assert_eq!(first_word_digit("2"), 2);
        assert_eq!(first_word_digit("two"), 2);
        assert_eq!(first_word_digit("3"), 3);
        assert_eq!(first_word_digit("three"), 3);
        assert_eq!(first_word_digit("4"), 4);
        assert_eq!(first_word_digit("four"), 4);
        assert_eq!(first_word_digit("5"), 5);
        assert_eq!(first_word_digit("five"), 5);
        assert_eq!(first_word_digit("6"), 6);
        assert_eq!(first_word_digit("six"), 6);
        assert_eq!(first_word_digit("7"), 7);
        assert_eq!(first_word_digit("seven"), 7);
        assert_eq!(first_word_digit("8"), 8);
        assert_eq!(first_word_digit("eight"), 8);
        assert_eq!(first_word_digit("9"), 9);
        assert_eq!(first_word_digit("nine"), 9);
    }

    #[test]
    fn gets_last_word_digits() {
        assert_eq!(last_word_digit(""), 0);
        assert_eq!(last_word_digit("0"), 0);
        assert_eq!(last_word_digit("zero"), 0);
        assert_eq!(last_word_digit("1"), 1);
        assert_eq!(last_word_digit("one"), 1);
        assert_eq!(last_word_digit("2"), 2);
        assert_eq!(last_word_digit("two"), 2);
        assert_eq!(last_word_digit("3"), 3);
        assert_eq!(last_word_digit("three"), 3);
        assert_eq!(last_word_digit("4"), 4);
        assert_eq!(last_word_digit("four"), 4);
        assert_eq!(last_word_digit("5"), 5);
        assert_eq!(last_word_digit("five"), 5);
        assert_eq!(last_word_digit("6"), 6);
        assert_eq!(last_word_digit("six"), 6);
        assert_eq!(last_word_digit("7"), 7);
        assert_eq!(last_word_digit("seven"), 7);
        assert_eq!(last_word_digit("8"), 8);
        assert_eq!(last_word_digit("eight"), 8);
        assert_eq!(last_word_digit("9"), 9);
        assert_eq!(last_word_digit("nine"), 9);
    }
}
