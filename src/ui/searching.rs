use super::components::*;
use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::Paragraph,
    Frame,
};

pub fn render(app: &App, frame: &mut Frame) {
    let screen = frame.area();
    let spinner = app.spinner_frames[app.spinner_index];
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

    let loading_paragraph = Paragraph::new(format!("Searching... {}", spinner))
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::ITALIC),
        );
    frame.render_widget(loading_paragraph, layout[1]);
}
