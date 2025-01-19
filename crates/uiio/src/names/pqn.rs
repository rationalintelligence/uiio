pub struct Pqn {
    components: Vec<String>,
}

impl Pqn {
    pub fn iter(&self) -> impl Iterator<Item = &'_ str> {
        self.components.iter().map(String::as_ref)
    }
}
