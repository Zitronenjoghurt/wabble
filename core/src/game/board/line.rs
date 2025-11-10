#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct BoardLines {
    vertical: Vec<BoardLineVertical>,
    horizontal: Vec<BoardLineHorizontal>,
}

impl BoardLines {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn extend_vertical(&mut self, x: u8, y: u8) {
        for line in &mut self.vertical {
            if line.extend_if_possible(x, y) {
                return;
            }
        }
        self.vertical.push(BoardLineVertical::new(y, y, x));
    }

    pub fn extend_horizontal(&mut self, x: u8, y: u8) {
        for line in &mut self.horizontal {
            if line.extend_if_possible(x, y) {
                return;
            }
        }
        self.horizontal.push(BoardLineHorizontal::new(x, x, y));
    }

    pub fn prune_single_lines(&mut self) {
        self.vertical.retain(|line| line.length() > 1);
        self.horizontal.retain(|line| line.length() > 1);
    }

    pub fn vertical_lines(&self) -> &[impl BoardLine] {
        &self.vertical
    }

    pub fn horizontal_lines(&self) -> &[impl BoardLine] {
        &self.horizontal
    }
}

pub trait BoardLine {
    fn extend_if_possible(&mut self, x: u8, y: u8) -> bool;
    fn length(&self) -> u8;
    fn contains(&self, x: u8, y: u8) -> bool;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoardLineVertical {
    start: u8,
    end: u8,
    column: u8,
}

impl BoardLineVertical {
    pub fn new(start: u8, end: u8, column: u8) -> Self {
        Self { start, end, column }
    }
}

impl BoardLine for BoardLineVertical {
    fn extend_if_possible(&mut self, x: u8, y: u8) -> bool {
        if x != self.column {
            return false;
        };

        if y == self.start.saturating_sub(1) {
            self.start = y;
            return true;
        } else if y == self.end.saturating_add(1) {
            self.end = y;
            return true;
        } else if y >= self.start && y <= self.end {
            return true;
        }

        false
    }

    fn length(&self) -> u8 {
        self.end.saturating_sub(self.start) + 1
    }

    fn contains(&self, x: u8, y: u8) -> bool {
        if self.column != x {
            return false;
        };
        y >= self.start && y <= self.end
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoardLineHorizontal {
    start: u8,
    end: u8,
    row: u8,
}

impl BoardLineHorizontal {
    pub fn new(start: u8, end: u8, row: u8) -> Self {
        Self { start, end, row }
    }
}

impl BoardLine for BoardLineHorizontal {
    fn extend_if_possible(&mut self, x: u8, y: u8) -> bool {
        if y != self.row {
            return false;
        };

        if x == self.start.saturating_sub(1) {
            self.start = x;
            return true;
        } else if x == self.end.saturating_add(1) {
            self.end = x;
            return true;
        } else if x >= self.start && x <= self.end {
            return true;
        }

        false
    }

    fn length(&self) -> u8 {
        self.end.saturating_sub(self.start) + 1
    }

    fn contains(&self, x: u8, y: u8) -> bool {
        if self.row != y {
            return false;
        };
        x >= self.start && x <= self.end
    }
}
