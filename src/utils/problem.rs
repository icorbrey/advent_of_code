use std::fs;

use inquire::{Select, Text};

const PART_1: &'static str = "Part One";
const PART_2: &'static str = "Part Two";

pub trait Problem {
    fn id(&self) -> &str;
    fn part_1(&self, input: &str);
    fn part_2(&self, input: &str);

    fn run(&self) {
        let options = vec![PART_1, PART_2];

        let part = Select::new("Part:", options)
            .prompt()
            .expect("Something went wrong.");
        let path = Text::new("Path:").prompt().expect("Something went wrong.");
        let file = fs::read_to_string(path).expect("File not found.");

        match part {
            PART_1 => self.part_1(file.as_str()),
            PART_2 => self.part_2(file.as_str()),
            _ => (),
        };
    }
}
