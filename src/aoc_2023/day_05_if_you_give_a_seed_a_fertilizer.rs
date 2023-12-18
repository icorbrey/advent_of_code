use regex::Regex;

use crate::utils::prelude::*;

pub struct IfYouGiveASeedAFertilizer;

impl Problem for IfYouGiveASeedAFertilizer {
    fn id(&self) -> &str {
        "Day 5: If You Give A Seed A Fertilizer"
    }

    fn part_1(&self, input: &str) {
        let seeds = load_seeds(input);
        let map_sets = load_map_sets(input);

        let location = seeds
            .iter()
            .map(|seed| map_sets.iter().fold(*seed, |x, map_set| map_set.convert(x)))
            .min()
            .unwrap();

        println!("Location: {}", location);
    }

    fn part_2(&self, input: &str) {
        let seed_pairs = load_seed_pairs(input);
        let map_sets = load_map_sets(input);

        let mut location: Seed = Seed::MAX;

        for (start, len) in seed_pairs {
            println!("Seed pair: {}, {}", start, len);
            'kek: for seed in start..(start + len) {
                let value = map_sets.iter().fold(seed, |x, map_set| map_set.convert(x));
                if value < location {
                    println!("New min location: {}", value);
                    location = value;
                    break 'kek;
                }
            }
        }

        println!("\nLocation: {}", location);
    }
}

type Seed = u64;

const MATCH_SEEDS: &'static str = r"seeds:((?:\s\d+)+)";
const MATCH_U32: &'static str = r"\d+";

fn load_seeds(input: &str) -> Vec<Seed> {
    (Regex::new(MATCH_SEEDS).ok())
        .and_then(|re| re.captures(input))
        .and_then(|cap| cap.get(1))
        .zip(Regex::new(MATCH_U32).ok())
        .and_then(|(cap, re)| {
            Some(
                re.find_iter(cap.as_str())
                    .filter_map(|l| l.as_str().parse().ok())
                    .collect(),
            )
        })
        .unwrap_or(vec![])
}

const MATCH_U32_PAIR: &'static str = r"(\d+) (\d+)";

fn load_seed_pairs(input: &str) -> Vec<(Seed, Seed)> {
    let input = (Regex::new(MATCH_SEEDS).ok())
        .and_then(|re| re.captures(input))
        .and_then(|cap| cap.get(1))
        .map(|cap| cap.as_str())
        .unwrap();

    (Regex::new(MATCH_U32_PAIR).ok())
        .map(|re| {
            re.captures_iter(input)
                .filter_map(|cap| {
                    (cap.get(1).and_then(|x| x.as_str().parse::<Seed>().ok()))
                        .zip(cap.get(2).and_then(|x| x.as_str().parse::<Seed>().ok()))
                })
                .collect()
        })
        .unwrap_or(vec![])
}

const MATCH_MAP_SET: &'static str = r"[a-z\-]+ map:\n((?:\d+ \d+ \d+\n)+)";

fn load_map_sets(input: &str) -> Vec<MapSet> {
    if let Some(re) = Regex::new(MATCH_MAP_SET).ok() {
        re.captures_iter(input)
            .map(|cap| cap.get(1).unwrap().as_str())
            .map(MapSet::from)
            .collect()
    } else {
        vec![]
    }
}

struct MapSet {
    maps: Vec<Map>,
}

impl MapSet {
    pub fn convert(&self, value: Seed) -> Seed {
        if let Some(map) = self.maps.iter().find(|m| m.is_match(value)) {
            map.convert(value)
        } else {
            value
        }
    }
}

impl From<&str> for MapSet {
    fn from(value: &str) -> Self {
        Self {
            maps: value
                .split('\n')
                .filter(|l| l.len() != 0)
                .map(Map::from)
                .collect(),
        }
    }
}

struct Map {
    dest: Seed,
    src: Seed,
    len: Seed,
}

impl Map {
    pub fn is_match(&self, value: Seed) -> bool {
        self.src <= value && value <= self.src + self.len
    }

    pub fn convert(&self, value: Seed) -> Seed {
        if self.src < self.dest {
            value + (self.dest - self.src)
        } else {
            value - (self.src - self.dest)
        }
    }
}

const MATCH_MAP: &'static str = r"(\d+) (\d+) (\d+)";

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let cap = (Regex::new(MATCH_MAP).ok())
            .and_then(|re| re.captures(value))
            .unwrap();

        Self {
            dest: cap.get(1).unwrap().as_str().parse().unwrap(),
            src: cap.get(2).unwrap().as_str().parse().unwrap(),
            len: cap.get(3).unwrap().as_str().parse().unwrap(),
        }
    }
}
