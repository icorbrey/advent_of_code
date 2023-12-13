use inquire::Select;

use super::year::Year;

pub struct AdventOfCode {
    years: Vec<Year>,
}

impl AdventOfCode {
    pub fn new() -> Self {
        Self { years: vec![] }
    }

    pub fn add_year(mut self, year: Year) -> Self {
        self.years.push(year);
        self
    }

    pub fn run(&self) {
        let options = self.years.iter().map(|year| year.id).collect();
        (Select::new("Year:", options).prompt().ok())
            .and_then(|id| self.years.iter().find(|y| y.id == id))
            .and_then(|y| Some(y.run()));
    }
}
