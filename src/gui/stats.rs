use tui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, BorderType, Borders},
    Frame,
};

pub fn ui<B: Backend>(f: &mut Frame<B>, chunk: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title("Statistics");
    f.render_widget(block, chunk);
}
