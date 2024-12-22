use std::fs;

use color_eyre::Result;
use ratatui::text::{Line, Span, Text};
use tokio::sync::mpsc;

mod ui;

struct Position {
    row: usize,
    column: usize,
}

impl Position {
    fn offset(&self, offset: &Offset) -> Self {
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

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let input = fs::read_to_string("input").unwrap();

    let grid: Text = input
        .lines()
        .map(|row| {
            row.chars()
                .map(|character| Span::raw(character.to_string()))
                .collect::<Line>()
        })
        .collect();

    let (ui_queue, ui_rx) = mpsc::channel(8);
    tokio::spawn(ui::ui_task(ui_rx, grid.clone())).await?

    // grid.iter().enumerate().for_each(|(row_index, row)| {
    //     row.iter()
    //         .enumerate()
    //         .for_each(|(column_index, character)| {
    //             let check_char_at_position = |expected: char, position: &Position| {
    //                 if grid[position.row][position.column] == expected {
    //                     Ok(())
    //                 } else {
    //                     Err(())
    //                 }
    //             };
    //             if character == &'X' {
    //                 let x_position = Position {
    //                     row: row_index,
    //                     column: column_index,
    //                 };
    //                 Position::directions().try_for_each(|direction| {
    //                     let m_position = x_position.offset(direction);
    //                     check_char_at_position('M', &m_position)?;
    //                     let a_position = m_position.offset(direction);
    //                     check_char_at_position('A', &a_position)?;
    //                     let s_position = a_position.offset(direction);
    //                     check_char_at_position('S', &s_position)
    //                 });
    //             }
    //         })
    // });
}
