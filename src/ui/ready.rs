use super::components::*;
use crate::app::App;
use crate::utils::StringExt;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub fn render(app: &mut App, frame: &mut Frame) {
    let screen = frame.area();

    // top bar
    let top_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(10), Constraint::Min(10)])
        .split(Rect {
            x: screen.x + 3,
            y: screen.y + 1,
            width: screen.width - 2,
            height: 3,
        });

    let small_gg_logo = google_logo_small();
    frame.render_widget(small_gg_logo, top_layout[0]);

    let search_box = create_input_box(app.input.as_str(), &app.input_mode, "Search", None, None);
    frame.render_widget(search_box, top_layout[1]);

    let second_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(2), Constraint::Min(0)])
        .split(Rect {
            x: screen.x,
            y: screen.y + 3,
            width: screen.width,
            height: screen.height.saturating_sub(3),
        });

    let help_msg = help_msg(
        &app.input_mode,
        app.debug_mode,
        app.cache.enable_cache_hit_notification,
    );
    frame.render_widget(help_msg, second_layout[0]);

    let search_area = second_layout[1];
    let items: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, msg)| {
            let highlighted_text = highlight_matches(&msg.description, &app.input);
            let lines = vec![
                Line::styled(format!("{} ", msg.url), Style::default().fg(Color::Cyan)),
                highlighted_text.lines[0].clone(),
                Line::styled(
                    format!("Crawl supported: {}", msg.url_supported_flag),
                    Style::default().fg(if msg.url_supported_flag {
                        Color::Green
                    } else {
                        Color::Red
                    }),
                ),
            ];
            let style = if i == app.selected_idx && app.input_mode == InputMode::Normal {
                Style::default().bg(Color::Blue).fg(Color::White)
            } else {
                Style::default()
            };
            ListItem::new(lines).style(style)
        })
        .collect();

    let results_title = if !app.messages.is_empty() {
        format!("Results for '{}'", app.input.clone())
    } else {
        "Search results".to_string()
    };
    let search_results_block = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(results_title))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

    frame.render_stateful_widget(
        search_results_block,
        search_area,
        &mut app.results_list_state.clone(),
    );
}

pub fn highlight_matches<'a>(text: &'a str, input: &'a str) -> Text<'a> {
    if input.is_empty() {
        return Text::from(text.to_string());
    }
    let mut highlighted_text = Text::default();
    let lower_input = input.to_lowercase();
    let mut start = 0;
    let txt_lower = text.to_lowercase();
    while let Some(pos) = txt_lower[start..].find(&lower_input) {
        let end = start + pos;
        highlighted_text.extend(Text::from(text[start..end].to_string()));
        highlighted_text.extend(Text::styled(
            &text[end..end + lower_input.grapheme_len()],
            Style::default().bg(Color::Yellow).fg(Color::Black),
        ));
        start = end + lower_input.grapheme_len();
    }
    highlighted_text.extend(Text::from(text[start..].to_string()));
    highlighted_text
}
