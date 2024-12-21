use std::fs;

use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{
    text::{Line, Span, Text},
    DefaultTerminal,
};

pub(crate) async fn ui_task() -> Result<()> {
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let input = fs::read_to_string("input").unwrap();

    let grid: Text = input
        .lines()
        .map(|row| {
            row.chars()
                .map(|character| Span::raw(character.to_string()))
                .collect::<Line>()
        })
        .collect();

    loop {
        terminal.draw(|frame| {
            frame.render_widget(&grid, frame.area());
        })?;
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    Ok(())
}
