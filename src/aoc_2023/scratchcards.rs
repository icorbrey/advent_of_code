use regex::Regex;

use crate::utils::{infra::Problem, io::prompt_for_file};

pub struct Scratchcards;

impl Problem for Scratchcards {
    fn id(&self) -> &str {
        "Day 4: Scratchcards"
    }

    fn run(&self) -> Result<(), ()> {
        let points: u32 = prompt_for_file()
            .split('\n')
            .map(Card::from)
            .map(|c| c.points())
            .sum();
        println!("{}", points);
        Ok(())
    }
}

struct Card {
    pub id: u32,
    pub winning_numbers: Vec<u32>,
    pub card_numbers: Vec<u32>,
}

impl Card {
    pub fn points(&self) -> u32 {
        let count = self
            .card_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count();
        match count {
            0 => 0,
            x => 2_u32.pow(x as u32 - 1),
        }
    }
}

const MATCH_CARD: &'static str = r"(.*):(.*)\|(.*)";

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let caps = Regex::new(MATCH_CARD).unwrap().captures(value).unwrap();

        Self {
            id: extract_numbers(caps.get(1).unwrap().as_str())[0],
            winning_numbers: extract_numbers(caps.get(2).unwrap().as_str()),
            card_numbers: extract_numbers(caps.get(3).unwrap().as_str()),
        }
    }
}

const MATCH_NUM: &'static str = r"(\d+)";

fn extract_numbers(text: &str) -> Vec<u32> {
    Regex::new(MATCH_NUM)
        .unwrap()
        .captures_iter(text)
        .map(|c| c.get(1).unwrap().as_str().parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_loads_cards() {
        let card = Card::from("Card 123: 12 34 56 78 90 |  1  2  3  4  5  6  7  8");
        assert_eq!(card.id, 123);
        assert_eq!(card.winning_numbers, vec![12, 34, 56, 78, 90]);
        assert_eq!(card.card_numbers, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
