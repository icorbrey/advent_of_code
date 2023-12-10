use inquire::Select;

pub struct Year {
    pub id: &'static str,
    problems: Vec<Box<dyn Problem>>,
}

impl Year {
    pub fn new(id: &'static str) -> Self {
        Self {
            id,
            problems: vec![],
        }
    }

    pub fn add_problem<P: Problem + 'static>(mut self, problem: P) -> Self {
        self.problems.push(Box::new(problem));
        self
    }

    pub fn run(&self) -> Result<(), ()> {
        match Select::new("Problem:", self.problems.iter().map(|p| p.id()).collect()).prompt() {
            Ok(problem) => match self.problems.iter().find(|p| p.id() == problem) {
                Some(problem) => problem.run(),
                None => Err(()),
            },
            Err(_) => Err(()),
        }
    }
}

pub trait Problem {
    fn id(&self) -> &str;
    fn run(&self) -> Result<(), ()>;
}
