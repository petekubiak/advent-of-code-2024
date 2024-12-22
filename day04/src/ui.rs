use std::sync::Arc;

use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{text::Text, DefaultTerminal};
use tokio::sync::Mutex;

pub(crate) async fn ui_task(grid: Arc<Mutex<Text<'_>>>) -> Result<()> {
    let terminal = ratatui::init();
    let result = run(terminal, grid);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, grid: Arc<Mutex<Text>>) -> Result<()> {
    loop {
        terminal.draw(|frame| {
            if let Ok(grid) = grid.clone().try_lock_owned() {
                frame.render_widget(&*grid, frame.area());
            }
        })?;
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break;
        }
    }
    Ok(())
}
