use regex::Regex;

use crate::utils::prelude::*;

pub struct Scratchcards;

impl Problem for Scratchcards {
    fn id(&self) -> &str {
        "Day 4: Scratchcards"
    }

    fn part_1(&self, input: &str) {
        let sum: u32 = load_cards(input).iter().map(Card::points).sum();
        println!("Sum: {}", sum);
    }

    fn part_2(&self, input: &str) {
        let mut cards = load_cards(input);

        for i in 0..cards.len() {
            let _cards = cards.clone();
            let cursor = _cards.get(i).unwrap();
            for j in (i + 1)..(cursor.matches() as usize + i + 1) {
                cards.get_mut(j).and_then(|c| Some(c.add(cursor.count)));
            }
        }

        let sum: u32 = cards.iter().map(|c| c.count).sum();
        println!("Sum: {}", sum);
    }
}

fn load_cards(input: &str) -> Vec<Card> {
    input
        .split('\n')
        .filter(|l| l.len() != 0)
        .map(Card::from)
        .collect()
}

#[derive(Debug, PartialEq)]
struct Card {
    pub id: u32,
    pub numbers: Vec<u32>,
    pub winning_numbers: Vec<u32>,
    pub count: u32,
}

impl Card {
    pub fn matches(&self) -> u32 {
        (self.numbers.iter())
            .filter(|x| self.winning_numbers.contains(x))
            .count() as u32
    }

    pub fn points(&self) -> u32 {
        match self.matches() {
            0 => 0,
            x => 2_u32.pow(x - 1),
        }
    }

    pub fn add(&mut self, quantity: u32) {
        self.count += quantity;
    }
}

impl Clone for Card {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            numbers: self.numbers.clone(),
            winning_numbers: self.winning_numbers.clone(),
            count: self.count.clone(),
        }
    }
}

const MATCH_CARD: &'static str = r"Card\s+(\d+):\s+((?:\d+\s+)+)\|((?:\s+\d+)+)";

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let cap = (Regex::new(MATCH_CARD).ok())
            .and_then(|re| re.captures(value))
            .expect("Match was not found.");

        Self {
            id: (cap.get(1))
                .and_then(|id| id.as_str().parse().ok())
                .expect("ID was not a number."),
            winning_numbers: (cap.get(2))
                .expect("Winning numbers were not found.")
                .as_str()
                .replace("  ", " ")
                .split(" ")
                .filter_map(|x| x.parse().ok())
                .collect(),
            numbers: (cap.get(3))
                .expect("Numbers were not found.")
                .as_str()
                .replace("  ", " ")
                .split(" ")
                .filter_map(|x| x.parse().ok())
                .collect(),
            count: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_cards_from_strings() {
        assert_eq!(
            Card::from("Card 123: 1 2 3 | 4 5 6"),
            Card {
                id: 123,
                numbers: vec![4, 5, 6],
                winning_numbers: vec![1, 2, 3],
                count: 1,
            }
        );
    }

    #[test]
    fn calculates_points() {
        assert_eq!(0, Card::from("Card 1: 1 | 0").points());
        assert_eq!(1, Card::from("Card 1: 1 | 1").points());
        assert_eq!(2, Card::from("Card 1: 1 2 | 1 2").points());
        assert_eq!(4, Card::from("Card 1: 1 2 3 | 1 2 3").points());
        assert_eq!(8, Card::from("Card 1: 1 2 3 4 | 1 2 3 4").points());
        assert_eq!(16, Card::from("Card 1: 1 2 3 4 5 | 1 2 3 4 5").points());
        assert_eq!(32, Card::from("Card 1: 1 2 3 4 5 6 | 1 2 3 4 5 6").points());
    }
}
