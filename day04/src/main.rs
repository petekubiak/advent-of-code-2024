mod types;
mod ui;

use std::fs;

use color_eyre::Result;
use ratatui::text::{Line, Span, Text};
use tokio::sync::mpsc::{self, Sender};
use types::{Offset, Position, Status, UpdateMessage};

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
                    Position::directions().for_each(|direction| {
                        send_update(&ui_queue, &x_position, direction, Status::Checking);

                        let m_position = x_position.offset(direction);
                        if check_char_at_position("M", &m_position).is_err() {
                            send_update(&ui_queue, &x_position, direction, Status::Invalid);
                            return;
                        }

                        let a_position = m_position.offset(direction);
                        if check_char_at_position("A", &a_position).is_err() {
                            send_update(&ui_queue, &x_position, direction, Status::Invalid);
                            return;
                        }

                        let s_position = a_position.offset(direction);
                        if check_char_at_position("S", &s_position).is_err() {
                            send_update(&ui_queue, &x_position, direction, Status::Invalid);
                            return;
                        }

                        send_update(&ui_queue, &x_position, direction, Status::Valid);
                    });
                }
            })
    });

    Ok(())
}

fn send_update(
    queue: &Sender<UpdateMessage>,
    position: &Position,
    direction: &Offset,
    status: Status,
) {
    let _ = queue.try_send((position.clone(), direction.clone(), status));
}
