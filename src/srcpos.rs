#[derive(Debug, Clone, Copy)]
pub struct SrcPos {
    line: usize,
    col: usize,
}

impl SrcPos {
    pub fn build() -> SrcPos {
        SrcPos {
            line: 1,
            col: 1,
        }
    }

    pub fn update(&mut self, ch: char) {
        match ch {
            '\n' => {
                self.line += 1;
                self.col = 1;
            }
            _ => self.col += 1,
        }
    }

    pub fn position(&self) -> (usize, usize) {
        (self.line, self.col)
    }
}
