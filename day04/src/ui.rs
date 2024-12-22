use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::text::Text;
use tokio::sync::mpsc::Receiver;

use crate::types::Position;

pub(crate) async fn ui_task(rx: Receiver<Position>, grid: Text<'_>) -> Result<()> {
    let mut terminal = ratatui::init();

    let result = loop {
        terminal.draw(|frame| {
            frame.render_widget(&grid, frame.area());
        })?;
        if matches!(event::read().expect("failed to read event"), Event::Key(_)) {
            break Ok(());
        }
    };

    ratatui::restore();
    result
}
