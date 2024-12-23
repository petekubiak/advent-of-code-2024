use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{style::Stylize, text::Text, DefaultTerminal};
use tokio::{select, sync::mpsc::Receiver};

use crate::types::{Offset, Position, Status};

pub(crate) async fn ui_task(
    mut rx: Receiver<(Position, Offset, Status)>,
    mut grid: Text<'_>,
) -> Result<()> {
    let mut terminal = ratatui::init();

    let result = loop {
        select! {
            Some((position, offset, status)) = rx.recv() => update_grid(&mut grid, position, offset, status).await,
            _ = watch_for_event() => break Ok(()),
        }
        draw(&mut terminal, &grid).await?;
    };

    ratatui::restore();
    result
}

async fn draw<'a>(terminal: &'a mut DefaultTerminal, grid: &Text<'_>) -> Result<()> {
    terminal.draw(|frame| {
        frame.render_widget(grid, frame.area());
    })?;
    Ok(())
}

async fn watch_for_event() {
    loop {
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
}

async fn update_grid(grid: &mut Text<'_>, position: Position, offset: Offset, status: Status) {
    let positions = (0..4).scan(position, |state, _| {
        let result = state.clone();
        state.offset(&offset);
        Some(result)
    });
    positions.for_each(|position| {
        let character = &mut grid.lines[position.row].spans[position.column];
        character.style = match status {
            Status::Checking => character.style.yellow(),
            Status::Invalid => character.style.white(),
            Status::Valid => character.style.white().on_green(),
        };
    })
}
