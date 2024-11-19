#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    line: usize,
    column: usize,
}

impl Location {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

impl Default for Location {
    fn default() -> Self {
        Self { line: 0, column: 0 }
    }
}