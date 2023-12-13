use regex::Regex;

use crate::utils::{infra::Problem, io::prompt_for_file, regex::StrRegexExtensions};

pub struct IfYouGiveASeedAFertilizer;

impl Problem for IfYouGiveASeedAFertilizer {
    fn id(&self) -> &str {
        "Day 5: If You Give A Seed A Fertilizer"
    }

    fn run(&self) -> Result<(), ()> {
        let file = prompt_for_file();
        let mut globs = file.split("map");

        let seeds = globs.next().and_then(|s| s.to_u32_vec().ok()).unwrap();

        Ok(())
    }
}

const MATCH_MAP_RANGE: &'static str = r"(\d+) (\d+) (\d+)";

struct MapRange {
    offset: i32,
    start: u32,
    end: u32,
}

impl MapRange {
    pub fn new(start: u32, end: u32, offset: i32) -> Self {
        Self { start, end, offset }
    }

    pub fn applies_to(&self, value: u32) -> bool {
        self.start <= value && value <= self.end
    }

    pub fn convert(&self, value: u32) -> u32 {
        (value as i32 + self.offset) as u32
    }
}

impl From<&str> for MapRange {
    fn from(value: &str) -> Self {
        let caps = Regex::new(MATCH_MAP_RANGE)
            .unwrap()
            .captures(value)
            .unwrap();

        let dest: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let src: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let len: u32 = caps.get(3).unwrap().as_str().parse().unwrap();

        Self {
            offset: dest as i32 - src as i32,
            end: src + len - 1,
            start: src,
        }
    }
}

#[cfg(test)]
mod map_range {
    use super::*;

    #[test]
    fn constructs_from_strings() {
        let map_range = MapRange::from("1 5 2");
        assert_eq!(5, map_range.start);
        assert_eq!(6, map_range.end);
        assert_eq!(-4, map_range.offset);
    }

    #[test]
    fn correctly_applies_to_numbers() {
        let map_range = MapRange::new(1, 2, 5);
        assert!(map_range.applies_to(1));
        assert!(map_range.applies_to(2));
        assert!(!map_range.applies_to(3));
    }

    #[test]
    fn converts_numbers() {
        let map_range = MapRange::new(1, 2, 5);
        assert_eq!(map_range.convert(1), 6);
        assert_eq!(map_range.convert(2), 7);
    }
}
