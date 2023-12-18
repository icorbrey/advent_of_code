use utils::prelude::*;

mod aoc_2023;
mod utils;

fn main() {
    AdventOfCode::new()
        .add_year(
            Year::new(2023)
                .add_problem(aoc_2023::Trebuchet)
                .add_problem(aoc_2023::CubeConundrum)
                .add_problem(aoc_2023::GearRatios)
                .add_problem(aoc_2023::Scratchcards)
                .add_problem(aoc_2023::IfYouGiveASeedAFertilizer)
                .add_problem(aoc_2023::WaitForIt),
        )
        .run();
}
