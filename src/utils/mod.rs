pub mod infra;
pub mod io;
pub mod regex;
pub mod str_manip;

mod advent_of_code;
mod problem;
mod year;

pub mod prelude {
    pub use super::advent_of_code::AdventOfCode;
    pub use super::problem::Problem;
    pub use super::year::Year;
}
