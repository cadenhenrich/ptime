use std::time::Duration;

use tui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, BorderType, Borders},
    Frame,
};

// TODO: Finish implementing struct
pub struct Statistics {
    cycles: u32,
    stretches: u32,
    breaks: u32,
    working: Duration,
    time: Duration
}

pub fn ui<B: Backend>(f: &mut Frame<B>, chunk: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Timer");
    f.render_widget(block, chunk);
}
