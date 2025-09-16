
#[derive(Default)]
pub struct Pipeline<T> {
    steps: Vec<Box<dyn Fn(T) -> Result<T, std::io::Error>>>,
}

impl<T> Pipeline<T> {

    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
        }
    }

    pub fn add<F>(&mut self, step: F)
    where
        F: Fn(T) -> Result<T, std::io::Error> + 'static,
    {
        self.steps.push(Box::new(step));
    }

    pub fn run(&self, mut input: T) -> Result<T, std::io::Error> {
        for step in &self.steps {
            input = step(input)?;

        }

        Ok(input)
    }

}


