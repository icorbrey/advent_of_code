use inquire::Select;

use super::problem::Problem;

pub struct Year {
    pub id: u32,
    problems: Vec<Box<dyn Problem>>,
}

impl Year {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            problems: vec![],
        }
    }

    pub fn add_problem<P: Problem + 'static>(mut self, problem: P) -> Self {
        self.problems.push(Box::new(problem));
        self
    }

    pub fn run(&self) {
        let options = self.problems.iter().map(|p| p.id()).collect();
        (Select::new("Problem:", options).prompt().ok())
            .and_then(|id| self.problems.iter().find(|p| p.id() == id))
            .and_then(|p| Some(p.run()));
    }
}
