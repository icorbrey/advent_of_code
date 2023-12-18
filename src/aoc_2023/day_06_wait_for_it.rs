use regex::Regex;

use crate::utils::prelude::Problem;

pub struct WaitForIt;

impl Problem for WaitForIt {
    fn id(&self) -> &str {
        "Day 6: Wait For It"
    }

    fn part_1(&self, input: &str) {
        let races = load_races(input);
    }

    fn part_2(&self, input: &str) {
        let races = load_races(input);
    }
}

struct Race {
    /// The race's duration, in milliseconds.
    time: u32,

    /// The race's record distance, in millimeters.
    distance: u32,
}

impl From<(&u32, &u32)> for Race {
    fn from(value: (&u32, &u32)) -> Self {
        let (time, distance) = value;
        Self {
            time: *time,
            distance: *distance,
        }
    }
}

fn load_races(input: &str) -> Vec<Race> {
    (get_times(input).and_then(parse_u32))
        .zip(get_distances(input).and_then(parse_u32))
        .map(|(t, d)| t.iter().zip(d.iter()).map(Race::from).collect())
        .expect("Could not load races.")
}

const MATCH_TIME: &'static str = r"Time:((?:\s+\d+))";

fn get_times(input: &str) -> Option<&str> {
    (Regex::new(MATCH_TIME).ok())
        .and_then(|re| re.captures(input))
        .and_then(|cap| cap.get(1))
        .map(|cap| cap.as_str())
}

const MATCH_DISTANCE: &'static str = r"Distance:((?:\s+\d+))";

fn get_distances(input: &str) -> Option<&str> {
    (Regex::new(MATCH_DISTANCE).ok())
        .and_then(|re| re.captures(input))
        .and_then(|cap| cap.get(1))
        .map(|cap| cap.as_str())
}

const MATCH_U32: &'static str = r"\d+";

fn parse_u32(input: &str) -> Option<Vec<u32>> {
    (Regex::new(MATCH_U32).ok()).map(|re| {
        re.find_iter(input)
            .filter_map(|x| x.as_str().parse().ok())
            .collect()
    })
}
