#[derive(Clone)]
pub struct Position {
    pub row: usize,
    pub column: usize,
}

impl Position {
    pub(crate) fn offset(&self, offset: &Offset) -> Self {
        Position {
            row: (self.row as isize).saturating_add(offset.row) as usize,
            column: (self.column as isize).saturating_add(offset.column) as usize,
        }
    }

    pub(crate) fn directions() -> DirectionIterator {
        DirectionIterator { index: 0 }
    }
}

#[derive(Clone)]
pub struct Offset {
    row: isize,
    column: isize,
}

impl Offset {
    pub fn none() -> Self {
        Self { row: 0, column: 0 }
    }
}

pub struct DirectionIterator {
    index: usize,
}

impl DirectionIterator {
    const DIRECTIONS: [Offset; 8] = [
        Offset { row: -1, column: 0 },
        Offset { row: -1, column: 1 },
        Offset { row: 0, column: 1 },
        Offset { row: 1, column: 1 },
        Offset { row: 1, column: 0 },
        Offset { row: 1, column: -1 },
        Offset { row: 0, column: -1 },
        Offset {
            row: -1,
            column: -1,
        },
    ];
}

impl Iterator for DirectionIterator {
    type Item = &'static Offset;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.index < Self::DIRECTIONS.len() {
            Some(&Self::DIRECTIONS[self.index])
        } else {
            None
        };
        self.index += 1;
        result
    }
}

pub(crate) enum Status {
    Checking,
    Invalid,
    Valid,
}

pub(crate) type UpdateMessage = (Position, Offset, Status);
