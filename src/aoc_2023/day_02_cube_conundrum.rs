//! # Day 2: Cube Conundrum
//!
//! ## Part 1
//!
//! A bag contains 12 red, 13 green, and 14 blue cubes. Games are played in which cubes are drawn
//! from the bag at random. For example:
//!
//! ```
//! Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
//! Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
//! Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
//! Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
//! Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
//! ```
//!
//! Games 1, 2, and 5 are possible with the bag we have, but 3 and 4 are not. Find the sum of the
//! IDs of the games which are possible with the bag we have.
//!
//! ## Part 2

use regex::Regex;

use crate::utils::prelude::*;

pub struct CubeConundrum;

impl Problem for CubeConundrum {
    fn id(&self) -> &str {
        "Day 2: Cube Conundrum"
    }

    fn part_1(&self, input: &str) {
        let bag = Set::new(12, 13, 14);
        let sum: u32 = load_games(input)
            .iter()
            .filter(|g| g.sets.iter().all(|s| s.fits_within(&bag)))
            .map(|g| g.id)
            .sum();
        println!("Sum: {}", sum);
    }

    fn part_2(&self, input: &str) {
        let sum: u32 = load_games(input)
            .iter()
            .map(Game::min_set)
            .map(Set::power)
            .sum();
        println!("Sum: {}", sum);
    }
}

fn load_games(input: &str) -> Vec<Game> {
    input
        .split('\n')
        .filter(|l| 0 < l.len())
        .map(Game::from)
        .collect()
}

struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    pub fn min_set(&self) -> Set {
        let r = self.sets.iter().map(|s| s.red).max().unwrap();
        let g = self.sets.iter().map(|s| s.green).max().unwrap();
        let b = self.sets.iter().map(|s| s.blue).max().unwrap();
        Set::new(r, g, b)
    }
}

const MATCH_GAME_ID: &'static str = r"Game (\d+)";

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        Self {
            id: match_u32(MATCH_GAME_ID, value),
            sets: value.split(';').map(Set::from).collect(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Set {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    fn fits_within(&self, other: &Set) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }

    fn power(self) -> u32 {
        self.red * self.green * self.blue
    }
}

const MATCH_RED: &'static str = r"(\d+) red";
const MATCH_GREEN: &'static str = r"(\d+) green";
const MATCH_BLUE: &'static str = r"(\d+) blue";

impl From<&str> for Set {
    fn from(value: &str) -> Self {
        Self {
            red: match_u32(MATCH_RED, value),
            green: match_u32(MATCH_GREEN, value),
            blue: match_u32(MATCH_BLUE, value),
        }
    }
}

fn match_u32(pat: &str, value: &str) -> u32 {
    (Regex::new(pat).ok())
        .and_then(|re| re.captures(value))
        .and_then(|cap| cap.get(1))
        .and_then(|x| x.as_str().parse().ok())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_sets_from_strings() {
        assert_eq!(Set::from(""), Set::new(0, 0, 0));
        assert_eq!(Set::from("1 red"), Set::new(1, 0, 0));
        assert_eq!(Set::from("1 green"), Set::new(0, 1, 0));
        assert_eq!(Set::from("1 blue"), Set::new(0, 0, 1));
        assert_eq!(Set::from("1 red, 2 green, 3 blue"), Set::new(1, 2, 3));
    }

    #[test]
    fn creates_games_from_strings() {
        let game = Game::from("Game 123: 1 red; 2 green; 3 blue");
        assert_eq!(game.id, 123);
        assert!(game.sets.contains(&Set::new(1, 0, 0)));
        assert!(game.sets.contains(&Set::new(0, 2, 0)));
        assert!(game.sets.contains(&Set::new(0, 0, 3)));
    }

    #[test]
    fn loads_games_from_file() {
        let file = "Game 123:\nGame 456:\nGame 789:\n";
        let games = load_games(file);
        assert_eq!(123, games[0].id);
        assert_eq!(456, games[1].id);
        assert_eq!(789, games[2].id);
    }

    #[test]
    fn fits_sets_within_other_sets() {
        assert!(Set::new(0, 0, 0).fits_within(&Set::new(0, 0, 0)));
        assert!(Set::new(0, 0, 0).fits_within(&Set::new(1, 1, 1)));
        assert!(!Set::new(1, 1, 1).fits_within(&Set::new(0, 0, 0)));
    }
}
