use inquire::Select;

mod aoc_2023;
mod utils;

fn main() -> Result<(), ()> {
    let years = vec![aoc_2023::load()];

    match Select::new("Year:", years.iter().map(|y| y.id).collect()).prompt() {
        Ok(year) => match years.iter().find(|y| y.id == year) {
            Some(year) => year.run(),
            None => Err(()),
        },
        Err(_) => Err(()),
    }
}
