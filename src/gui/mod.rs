pub mod stats;
pub mod timer;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    Frame, widgets::{Block, Borders, BorderType},
};

pub fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(10)
        .constraints([Constraint::Max(10000), Constraint::Length(3)].as_ref())
        .split(f.size());
    let split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[0]);

    stats::ui(f, split[0]);
    timer::ui(f, split[1]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    f.render_widget(block, chunks[1]);
}
