#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl From<String> for Organization {
    fn from(value: String) -> Self {
        Organization::new(&value)
    }
}
