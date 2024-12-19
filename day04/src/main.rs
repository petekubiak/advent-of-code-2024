use std::fs;

struct Position {
    row: usize,
    column: usize,
}

impl Position {
    fn offset(self, offset: &Offset) -> Self {
        Position {
            row: (self.row as isize).saturating_add(offset.row) as usize,
            column: (self.column as isize).saturating_add(offset.column) as usize,
        }
    }

    fn directions() -> DirectionIterator {
        DirectionIterator { index: 0 }
    }
}

#[derive(Clone)]
struct Offset {
    row: isize,
    column: isize,
}

struct DirectionIterator {
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

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let grid: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();

    grid.iter().enumerate().for_each(|(row_index, row)| {
        row.iter()
            .enumerate()
            .for_each(|(column_index, character)| {
                let position = Position {
                    row: row_index,
                    column: column_index,
                };
                if character == "X" {
                    Position::directions().try_for_each(|direction| )
                }
            })
    });
}
