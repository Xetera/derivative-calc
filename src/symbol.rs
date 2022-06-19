#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Symbol(char);

impl Symbol {
    pub fn of_character(char: char) -> Self {
        Self(char)
    }
}

impl ToString for Symbol {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
