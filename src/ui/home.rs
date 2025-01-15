use super::components::*;
use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::Paragraph,
    Frame,
};

pub fn render(app: &mut App, frame: &mut Frame) {
    let screen = frame.area();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Length(3),
            Constraint::Percentage(40),
        ])
        .split(screen);

    let ascii_google = google_logo_ascii();
    let google_logo = Paragraph::new(ascii_google)
        .alignment(Alignment::Center)
        .style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_widget(google_logo, layout[0]);

    let search_box = create_input_box(app.input.as_str(), &app.input_mode, "search", None, None);
    frame.render_widget(search_box, layout[1]);

    if app.input_mode == InputMode::Editing {
        frame.set_cursor_position((layout[1].x + (app.cursor_idx as u16) + 1, layout[1].y + 1));
    }
}
