use regex::Regex;

use crate::utils::prelude::*;

pub struct GearRatios;

impl Problem for GearRatios {
    fn id(&self) -> &str {
        "Day 3: Gear Ratios"
    }

    fn part_1(&self, input: &str) {
        let globs = load_globs(input);

        let symbols: Vec<Symbol> = globs.iter().filter_map(Glob::symbol).collect();
        let numbers: Vec<Number> = globs.iter().filter_map(Glob::number).collect();

        let sum: u32 = numbers
            .iter()
            .filter(|n| symbols.iter().any(|s| s.bounds.is_adjacent_to(&n.bounds)))
            .map(|n| n.value)
            .sum();

        println!("Sum: {}", sum);
    }

    fn part_2(&self, input: &str) {
        let globs = load_globs(input);

        let symbols: Vec<Symbol> = globs.iter().filter_map(Glob::symbol).collect();
        let numbers: Vec<Number> = globs.iter().filter_map(Glob::number).collect();

        let sum: u32 = symbols
            .iter()
            .filter(|s| s.value == '*')
            .map(|s| {
                numbers
                    .iter()
                    .filter(|n| s.bounds.is_adjacent_to(&n.bounds))
                    .map(|n| n.value)
                    .collect::<Vec<u32>>()
            })
            .filter(|nums| nums.len() == 2)
            .map(|nums| nums.iter().product::<u32>())
            .sum();

        println!("Sum: {}", sum);
    }
}

fn load_globs(input: &str) -> Vec<Glob> {
    input
        .split('\n')
        .enumerate()
        .flat_map(Glob::from_line)
        .collect()
}

enum Glob {
    Number(Number),
    Symbol(Symbol),
}

const MATCH_NUMBER: &'static str = r"\d+";
const MATCH_SYMBOL: &'static str = r"[\*\$\+\-\=\/\@\%\&\#]";

impl Glob {
    pub fn from_line((y, line): (usize, &str)) -> Vec<Glob> {
        let mut globs = vec![];
        if let Ok(re) = Regex::new(MATCH_NUMBER) {
            let mut numbers = re
                .find_iter(line)
                .map(|c| {
                    Glob::Number(Number {
                        value: c.as_str().parse().unwrap(),
                        bounds: Bounds {
                            x: c.start(),
                            l: c.len(),
                            y,
                        },
                    })
                })
                .collect::<Vec<Glob>>();
            globs.append(&mut numbers);
        }
        if let Ok(re) = Regex::new(MATCH_SYMBOL) {
            let mut symbols = re
                .find_iter(line)
                .map(|c| {
                    Glob::Symbol(Symbol {
                        value: c.as_str().chars().next().unwrap(),
                        bounds: Bounds {
                            x: c.start(),
                            l: c.len(),
                            y,
                        },
                    })
                })
                .collect::<Vec<Glob>>();
            globs.append(&mut symbols);
        }
        globs
    }

    pub fn number(&self) -> Option<Number> {
        match self {
            Glob::Number(value) => Some(*value),
            _ => None,
        }
    }

    pub fn symbol(&self) -> Option<Symbol> {
        match self {
            Glob::Symbol(value) => Some(*value),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
struct Number {
    value: u32,
    bounds: Bounds,
}

#[derive(Clone, Copy)]
struct Symbol {
    value: char,
    bounds: Bounds,
}

#[derive(Clone, Copy)]
struct Bounds {
    x: usize,
    y: usize,
    l: usize,
}

impl Bounds {
    pub fn is_adjacent_to(&self, other: &Self) -> bool {
        (self.x - other.l <= other.x && other.x <= self.x + self.l)
            && (self.y - 1 <= other.y && other.y <= self.y + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_bounds_adjacency() {
        assert!(Bounds { x: 1, y: 1, l: 1 }.is_adjacent_to(&Bounds { x: 0, y: 1, l: 1 }));
        assert!(Bounds { x: 1, y: 1, l: 1 }.is_adjacent_to(&Bounds { x: 1, y: 1, l: 1 }));
        assert!(Bounds { x: 1, y: 1, l: 1 }.is_adjacent_to(&Bounds { x: 2, y: 1, l: 1 }));
        assert!(!Bounds { x: 1, y: 1, l: 1 }.is_adjacent_to(&Bounds { x: 3, y: 1, l: 1 }));
    }

    #[test]
    fn creates_numbers_from_strings() {
        let globs = Glob::from_line((0, ""));

        assert_eq!(0, globs.len());

        let globs = Glob::from_line((67, "12....345..*/"));

        assert_eq!(4, globs.len());
        assert_eq!(2, globs.iter().filter_map(Glob::number).count());
        assert_eq!(2, globs.iter().filter_map(Glob::symbol).count());

        let number_1 = globs[0].number().unwrap();

        assert_eq!(12, number_1.value);
        assert_eq!(0, number_1.bounds.x);
        assert_eq!(67, number_1.bounds.y);
        assert_eq!(2, number_1.bounds.l);

        let number_2 = globs[1].number().unwrap();

        assert_eq!(345, number_2.value);
        assert_eq!(6, number_2.bounds.x);
        assert_eq!(67, number_2.bounds.y);
        assert_eq!(3, number_2.bounds.l);

        let symbol_1 = globs[2].symbol().unwrap();

        assert_eq!('*', symbol_1.value);
        assert_eq!(11, symbol_1.bounds.x);
        assert_eq!(67, symbol_1.bounds.y);
        assert_eq!(1, symbol_1.bounds.l);

        let symbol_2 = globs[3].symbol().unwrap();

        assert_eq!('/', symbol_2.value);
        assert_eq!(12, symbol_2.bounds.x);
        assert_eq!(67, symbol_2.bounds.y);
        assert_eq!(1, symbol_2.bounds.l);
    }
}
