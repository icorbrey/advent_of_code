//! Day 3: Gear Ratios

use regex::Regex;

use crate::utils::{infra::Problem, io::prompt_for_file};

pub struct GearRatios;

impl Problem for GearRatios {
    fn id(&self) -> &str {
        "Day 3: Gear Ratios"
    }

    fn run(&self) -> Result<(), ()> {
        let globs = ingest_file(&prompt_for_file());
        let symbols = globs.iter().filter_map(Glob::symbol);
        let sum: u32 = globs
            .iter()
            .filter_map(Glob::number)
            .filter(|(_, n_bounds)| {
                symbols
                    .clone()
                    .any(|s_bounds| s_bounds.is_adjacent_to(*n_bounds))
            })
            .map(|(x, _)| x)
            .sum();
        println!("{}", sum);
        Ok(())
    }
}

fn ingest_file(text: &str) -> Vec<Glob> {
    text.split('\n').enumerate().flat_map(ingest_line).collect()
}

const MATCH_NUMBER: &'static str = r"(\d+)";
const MATCH_SYMBOL: &'static str = r"([\*\$\+\-\=\/\@\%\&\#])";

fn ingest_line((y, line): (usize, &str)) -> Vec<Glob> {
    let mut globs = vec![];
    for c in Regex::new(MATCH_NUMBER).unwrap().find_iter(line) {
        globs.push(Glob::Number((
            c.as_str().parse().unwrap(),
            BoundingBox::new(c.start(), y, c.len()),
        )));
    }
    for c in Regex::new(MATCH_SYMBOL).unwrap().find_iter(line) {
        globs.push(Glob::Symbol(BoundingBox::new(c.start(), y, 1)));
    }
    globs
}

#[derive(Debug, PartialEq)]
enum Glob {
    Number((u32, BoundingBox)),
    Symbol(BoundingBox),
}

impl Glob {
    pub fn number(&self) -> Option<(u32, BoundingBox)> {
        match self {
            Glob::Number(value) => Some(*value),
            _ => None,
        }
    }

    pub fn symbol(&self) -> Option<BoundingBox> {
        match self {
            Glob::Symbol(value) => Some(*value),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct BoundingBox {
    pub x: usize,
    pub y: usize,
    pub l: usize,
}

impl BoundingBox {
    pub fn new(x: usize, y: usize, l: usize) -> Self {
        Self { x, y, l }
    }

    pub fn is_adjacent_to(&self, other: Self) -> bool {
        assert_eq!(1, self.l);
        (self.x - other.l <= other.x && other.x <= self.x + 1)
            && (self.y - 1 <= other.y && other.y <= self.y + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_ingests_lines() {
        let y = 123;
        let globs = ingest_line((y, "123..*456.....789/"));
        assert!(globs.contains(&Glob::Symbol(BoundingBox::new(5, y, 1))));
        assert!(globs.contains(&Glob::Symbol(BoundingBox::new(17, y, 1))));
        assert!(globs.contains(&Glob::Number((123, BoundingBox::new(0, y, 3)))));
        assert!(globs.contains(&Glob::Number((456, BoundingBox::new(6, y, 3)))));
        assert!(globs.contains(&Glob::Number((789, BoundingBox::new(14, y, 3)))));
    }

    #[test]
    fn it_tests_bounding_boxes() {
        assert!(BoundingBox::new(3, 1, 1).is_adjacent_to(BoundingBox::new(0, 0, 3)));
        assert!(BoundingBox::new(3, 1, 1).is_adjacent_to(BoundingBox::new(0, 1, 3)));
        assert!(BoundingBox::new(3, 1, 1).is_adjacent_to(BoundingBox::new(0, 2, 3)));
        assert!(BoundingBox::new(3, 1, 1).is_adjacent_to(BoundingBox::new(1, 0, 3)));
        assert!(BoundingBox::new(3, 1, 1).is_adjacent_to(BoundingBox::new(1, 1, 3)));
        assert!(BoundingBox::new(3, 1, 1).is_adjacent_to(BoundingBox::new(1, 2, 3)));
        assert!(BoundingBox::new(3, 1, 1).is_adjacent_to(BoundingBox::new(2, 0, 3)));
        assert!(BoundingBox::new(3, 1, 1).is_adjacent_to(BoundingBox::new(2, 1, 3)));
        assert!(BoundingBox::new(3, 1, 1).is_adjacent_to(BoundingBox::new(2, 2, 3)));
        assert!(BoundingBox::new(3, 1, 1).is_adjacent_to(BoundingBox::new(3, 0, 3)));
        assert!(BoundingBox::new(3, 1, 1).is_adjacent_to(BoundingBox::new(3, 1, 3)));
        assert!(BoundingBox::new(3, 1, 1).is_adjacent_to(BoundingBox::new(3, 2, 3)));
        assert!(BoundingBox::new(3, 1, 1).is_adjacent_to(BoundingBox::new(4, 0, 3)));
        assert!(BoundingBox::new(3, 1, 1).is_adjacent_to(BoundingBox::new(4, 1, 3)));
        assert!(BoundingBox::new(3, 1, 1).is_adjacent_to(BoundingBox::new(4, 2, 3)));
        assert!(!BoundingBox::new(3, 1, 1).is_adjacent_to(BoundingBox::new(5, 0, 3)));
        assert!(!BoundingBox::new(3, 1, 1).is_adjacent_to(BoundingBox::new(5, 1, 3)));
        assert!(!BoundingBox::new(3, 1, 1).is_adjacent_to(BoundingBox::new(5, 2, 3)));
    }
}
