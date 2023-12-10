//! # Advent of Code 2023

use crate::{aoc_2023::trebuchet::Trebuchet, utils::infra::Year};

mod trebuchet;

pub fn load() -> Year {
    Year::new("2023").add_problem(Trebuchet)
}
