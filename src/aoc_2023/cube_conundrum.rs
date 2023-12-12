//! # Day 2: Cube Conundrum

use regex::Regex;

use crate::utils::{infra::Problem, io::prompt_for_file};

pub struct CubeConundrum;

impl Problem for CubeConundrum {
    fn id(&self) -> &str {
        "Day 2: Cube Conundrum"
    }

    fn run(&self) -> Result<(), ()> {
        let bag = Quantities::new(12, 13, 14);
        let sum: u32 = prompt_for_file()
            .split('\n')
            .map(Game::from)
            .map(|game| game.min_set().power())
            .sum();
        println!("{}", sum);
        Ok(())
    }
}

struct Game {
    pub id: u32,
    pub sets: Vec<Quantities>,
}

impl Game {
    pub fn is_possible_within(&self, bag: &Quantities) -> bool {
        self.sets.iter().all(|set| set.is_smaller_than(bag))
    }

    pub fn min_set(&self) -> Quantities {
        let r = self.sets.iter().map(|q| q.r).max().unwrap();
        let g = self.sets.iter().map(|q| q.g).max().unwrap();
        let b = self.sets.iter().map(|q| q.b).max().unwrap();
        Quantities::new(r, g, b)
    }
}

const MATCH_GAME: &'static str = r"Game (\d+)";

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let mut globs = value.split(':');
        let game_glob = globs.next().unwrap();
        let sets_glob = globs.next().unwrap();
        Self {
            id: Regex::new(MATCH_GAME).unwrap().captures(game_glob).unwrap()[1]
                .parse()
                .unwrap(),
            sets: sets_glob.split(';').map(Quantities::from).collect(),
        }
    }
}

#[derive(Debug)]
struct Quantities {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

impl Quantities {
    pub fn new(r: u32, g: u32, b: u32) -> Self {
        Self { r, g, b }
    }

    pub fn is_smaller_than(&self, bag: &Quantities) -> bool {
        self.r <= bag.r && self.g <= bag.g && self.b <= bag.b
    }

    pub fn power(&self) -> u32 {
        self.r * self.g * self.b
    }
}

impl PartialEq for Quantities {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

const MATCH_RED: &'static str = r"(\d+) red";
const MATCH_GREEN: &'static str = r"(\d+) green";
const MATCH_BLUE: &'static str = r"(\d+) blue";

impl From<&str> for Quantities {
    fn from(value: &str) -> Self {
        Self {
            r: match Regex::new(MATCH_RED).unwrap().captures(value) {
                Some(caps) => caps[1].parse().unwrap(),
                None => 0,
            },
            g: match Regex::new(MATCH_GREEN).unwrap().captures(value) {
                Some(caps) => caps[1].parse().unwrap(),
                None => 0,
            },
            b: match Regex::new(MATCH_BLUE).unwrap().captures(value) {
                Some(caps) => caps[1].parse().unwrap(),
                None => 0,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_tests_quantities() {
        assert!(Quantities::new(0, 0, 0).is_smaller_than(&Quantities::new(0, 0, 0)));
        assert!(Quantities::new(0, 0, 0).is_smaller_than(&Quantities::new(1, 0, 0)));
        assert!(Quantities::new(0, 0, 0).is_smaller_than(&Quantities::new(0, 1, 0)));
        assert!(Quantities::new(0, 0, 0).is_smaller_than(&Quantities::new(0, 0, 1)));
        assert!(Quantities::new(0, 0, 0).is_smaller_than(&Quantities::new(1, 1, 1)));
        assert!(!Quantities::new(2, 1, 1).is_smaller_than(&Quantities::new(1, 1, 1)));
        assert!(!Quantities::new(1, 2, 1).is_smaller_than(&Quantities::new(1, 1, 1)));
        assert!(!Quantities::new(1, 1, 2).is_smaller_than(&Quantities::new(1, 1, 1)));
    }

    #[test]
    fn it_creates_quantities() {
        assert_eq!(Quantities::new(0, 0, 0), Quantities::from(""));
        assert_eq!(Quantities::new(1, 0, 0), Quantities::from("1 red"));
        assert_eq!(Quantities::new(0, 1, 0), Quantities::from("1 green"));
        assert_eq!(Quantities::new(0, 0, 1), Quantities::from("1 blue"));
        assert_eq!(
            Quantities::new(1, 2, 3),
            Quantities::from("1 red, 2 green, 3 blue")
        );
    }

    #[test]
    fn it_creates_games() {
        let game = Game::from("Game 123: 1 red; 2 green; 3 blue");
        assert_eq!(game.id, 123);
        assert_eq!(game.sets[0], Quantities::new(1, 0, 0));
        assert_eq!(game.sets[1], Quantities::new(0, 2, 0));
        assert_eq!(game.sets[2], Quantities::new(0, 0, 3));
    }

    #[test]
    fn it_finds_minimum_sets() {
        let game = Game::from("Game 1: 1 red, 1 green, 1 blue");
        assert_eq!(game.min_set(), Quantities::new(1, 1, 1));

        let game = Game::from("Game 1: 1 red; 1 green; 1 blue");
        assert_eq!(game.min_set(), Quantities::new(1, 1, 1));

        let game = Game::from("Game 1: 1 red; 1 red, 2 green; 1 blue");
        assert_eq!(game.min_set(), Quantities::new(1, 2, 1));

        let game = Game::from("Game 1: 1 red; 1 red, 2 green");
        assert_eq!(game.min_set(), Quantities::new(1, 2, 0));
    }
}
