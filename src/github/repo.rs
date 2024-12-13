#[derive(Debug, Clone)]
pub struct Repo {
    name: String,
}

impl Repo {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
