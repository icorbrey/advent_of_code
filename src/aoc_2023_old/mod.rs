//! # Advent of Code 2023

use crate::utils::infra::Year;

use self::cube_conundrum::CubeConundrum;
use self::day_01_trebuchet::Trebuchet;
use self::gear_ratios::GearRatios;
use self::if_you_give_a_seed_a_fertilizer::IfYouGiveASeedAFertilizer;
use self::scratchcards::Scratchcards;

mod cube_conundrum;
mod day_01_trebuchet;
mod gear_ratios;
mod if_you_give_a_seed_a_fertilizer;
mod scratchcards;

pub fn load() -> Year {
    Year::new("2023")
        .add_problem(Trebuchet)
        .add_problem(CubeConundrum)
        .add_problem(GearRatios)
        .add_problem(Scratchcards)
        .add_problem(IfYouGiveASeedAFertilizer)
}
