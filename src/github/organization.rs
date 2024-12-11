#[derive(Debug, Clone)]
pub struct Organization {
    name: String,
}

impl Organization {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
