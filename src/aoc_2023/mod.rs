//! # Advent of Code 2023

use crate::utils::infra::Year;

use self::cube_conundrum::CubeConundrum;
use self::gear_ratios::GearRatios;
use self::scratchcards::Scratchcards;
use self::trebuchet::Trebuchet;

mod cube_conundrum;
mod gear_ratios;
mod scratchcards;
mod trebuchet;

pub fn load() -> Year {
    Year::new("2023")
        .add_problem(Trebuchet)
        .add_problem(CubeConundrum)
        .add_problem(GearRatios)
        .add_problem(Scratchcards)
}
