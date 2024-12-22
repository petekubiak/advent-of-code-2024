mod types;
mod ui;

use std::fs;

use color_eyre::Result;
use ratatui::text::{Line, Span, Text};
use tokio::sync::mpsc;
use types::Position;

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
    tokio::spawn(ui::ui_task(ui_rx, grid.clone())).await?;

    grid.iter().enumerate().for_each(|(row_index, row)| {
        row.iter()
            .enumerate()
            .for_each(|(column_index, character)| {
                let check_char_at_position = |expected, position: &Position| {
                    if grid.lines[position.row].spans[position.column].content == expected {
                        Ok(())
                    } else {
                        Err(())
                    }
                };
                if character.content == "X" {
                    let x_position = Position {
                        row: row_index,
                        column: column_index,
                    };
                    Position::directions().try_for_each(|direction| {
                        let m_position = x_position.offset(direction);
                        check_char_at_position("M", &m_position)?;
                        let a_position = m_position.offset(direction);
                        check_char_at_position("A", &a_position)?;
                        let s_position = a_position.offset(direction);
                        check_char_at_position("S", &s_position)
                    });
                }
            })
    });

    Ok(())
}
