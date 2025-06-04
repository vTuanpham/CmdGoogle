use super::components::*;
use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render(app: &App, frame: &mut Frame) {
    let screen = frame.area();
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(2), Constraint::Min(0)])
        .split(screen);

    let help = help_msg(&InputMode::Normal, app.debug_mode, app.cache.enable_cache_hit_notification);
    frame.render_widget(help, layout[0]);

    let content = app.crawl_content.as_deref().unwrap_or("");
    let paragraph = Paragraph::new(Text::from(content.to_string()))
        .block(Block::default().borders(Borders::ALL).title("Page"));
    frame.render_widget(paragraph, layout[1]);
}
