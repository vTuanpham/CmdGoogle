use ratatui::{
    layout::{Alignment, Rect},
    prelude::Buffer,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum InputMode {
    Normal,
    Editing,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum DisplayMode {
    Home,
    Searching,
    Ready,
}

#[derive(Default, derive_setters::Setters, Clone)]
pub struct Popup<'a> {
    #[setters(into)]
    pub title: Line<'a>,
    #[setters(into)]
    pub content: Text<'a>,
    pub border_style: Style,
    pub title_style: Style,
    pub style: Style,
}

impl<'a> Popup<'a> {
    pub fn new(title: impl Into<Line<'a>>, content: impl Into<Text<'a>>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            border_style: Style::default(),
            title_style: Style::default(),
            style: Style::default(),
        }
    }

    pub fn with_styles(mut self, border_style: Style, title_style: Style, style: Style) -> Self {
        self.border_style = border_style;
        self.title_style = title_style;
        self.style = style;
        self
    }

    pub fn calculate_area(&self, frame_area: Rect) -> Rect {
        let content_length = self.content.width();
        let width = (content_length + 4).min(frame_area.width as usize) as u16;
        let height = (self.content.height() + 4).min(frame_area.height as usize) as u16;
        Rect {
            x: (frame_area.width - width) / 2,
            y: (frame_area.height - height) / 2,
            width,
            height,
        }
    }

    pub fn calculate_top_right_area(&self, frame_area: Rect) -> Rect {
        let content_length = self.content.width();
        let width = (content_length + 4).min(frame_area.width as usize) as u16;
        let height = (self.content.height() + 4).min(frame_area.height as usize) as u16;
        let x = frame_area.x + frame_area.width.saturating_sub(width) - 1;
        let y = frame_area.y;
        Rect {
            x,
            y,
            width,
            height,
        }
    }
}

impl Widget for Popup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        let block = Block::new()
            .title(self.title)
            .title_style(self.title_style)
            .borders(Borders::ALL)
            .border_style(self.border_style);
        Paragraph::new(self.content)
            .wrap(Wrap { trim: true })
            .style(self.style)
            .block(block)
            .render(area, buf);
    }
}

pub fn create_warning_popup(input: &str) -> Popup {
    Popup::new("Warning", format!("No search result found for '{}'", input)).with_styles(
        Style::new().red(),
        Style::new().red(),
        Style::new().light_red(),
    )
}

pub fn create_error_popup(message: &str) -> Popup {
    Popup::new("Error", message).with_styles(
        Style::new().red(),
        Style::new().red().add_modifier(Modifier::BOLD),
        Style::new().light_red(),
    )
}

pub fn create_cache_popup() -> Popup<'static> {
    Popup::new("Cache", "Cached result used!").with_styles(
        Style::new().green(),
        Style::new().green(),
        Style::new().light_green(),
    )
}

pub fn create_history_popup(
    history_items: &[String],
    selected: usize,
    area_height: usize,
) -> Popup<'static> {
    let start_index = if history_items.len() <= area_height {
        0
    } else {
        selected
            .saturating_sub(area_height / 2)
            .min(history_items.len().saturating_sub(area_height))
    };

    let end_index = if history_items.len() <= area_height {
        history_items.len()
    } else {
        (start_index + area_height).min(history_items.len())
    };

    let mut lines = String::new();
    for (i, item) in history_items
        .iter()
        .enumerate()
        .skip(start_index)
        .take(end_index - start_index)
    {
        if i == selected {
            lines.push_str(&format!("> {}\n", item));
        } else {
            lines.push_str(&format!("  {}\n", item));
        }
    }
    Popup::new("History", lines).with_styles(
        Style::new().blue(),
        Style::new().blue(),
        Style::new().light_blue(),
    )
}

pub fn google_logo_small() -> Paragraph<'static> {
    // small color version
    let google_color = Line::from(vec![
        Span::styled(
            "g",
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "o",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "o",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "g",
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "l",
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "e",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
    ]);
    Paragraph::new(google_color).alignment(Alignment::Left)
}

pub fn google_logo_ascii() -> Text<'static> {
    // ASCII art
    Text::from(vec![
        Line::from(vec![Span::styled(
            " ____                   _      ",
            Style::default().fg(Color::Blue),
        )]),
        Line::from(vec![Span::styled(
            " / ___| ___   ___   __ _| | ___ ",
            Style::default().fg(Color::Red),
        )]),
        Line::from(vec![Span::styled(
            "| |  _ / _ \\ / _ \\ / _` | |/ _ \\",
            Style::default().fg(Color::Yellow),
        )]),
        Line::from(vec![Span::styled(
            "| |_| | (_) | (_) | (_| | |  __/",
            Style::default().fg(Color::Blue),
        )]),
        Line::from(vec![Span::styled(
            " \\____|\\___/ \\___/ \\__, |_|\\___|",
            Style::default().fg(Color::Green),
        )]),
        Line::from(vec![Span::styled(
            "                   |___/        ",
            Style::default().fg(Color::Red),
        )]),
    ])
}

// Create input box with a title
// For example:
// create_input_box("input", &InputMode::Editing, "Search")
pub fn create_input_box<'a>(
    input: &'a str,
    input_mode: &InputMode,
    title: &'a str,
    edit_style: Option<Style>,
    normal_style: Option<Style>,
) -> Paragraph<'a> {
    // Use default styles if None is provided
    let edit_style = edit_style.unwrap_or_else(|| Style::default().fg(Color::Yellow));
    let normal_style = normal_style.unwrap_or_default();

    let box_block = Block::default().borders(Borders::ALL).title(title);
    let input_box =
        Paragraph::new(input)
            .block(box_block)
            .style(if *input_mode == InputMode::Editing {
                edit_style
            } else {
                normal_style
            });

    input_box
}

pub fn help_msg(
    input_mode: &InputMode,
    debug_mode: bool,
    enable_cache_hit_notification: bool,
) -> Paragraph {
    let base_style = Style::default();
    let key_style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);
    let action_style = Style::default().fg(Color::Cyan);
    let status_style = Style::default().fg(Color::LightBlue);
    let separator_style = Style::default().fg(Color::DarkGray);

    let help_text = match input_mode {
        InputMode::Normal => {
            let spans = vec![
                Span::styled(
                    "[Normal] ",
                    base_style.fg(Color::Blue).add_modifier(Modifier::BOLD),
                ),
                Span::styled("q", key_style),
                Span::styled("=", separator_style),
                Span::styled("quit", action_style),
                Span::styled(" | ", separator_style),
                Span::styled("e", key_style),
                Span::styled("=", separator_style),
                Span::styled("edit", action_style),
                Span::styled(" | ", separator_style),
                Span::styled("c", key_style),
                Span::styled("=", separator_style),
                Span::styled("clear", action_style),
                Span::styled(" | ", separator_style),
                Span::styled("o", key_style),
                Span::styled("=", separator_style),
                Span::styled("open", action_style),
                Span::styled(" | ", separator_style),
                Span::styled("d", key_style),
                Span::styled("=", separator_style),
                Span::styled("debug", action_style),
                Span::styled("(", separator_style),
                Span::styled(
                    if debug_mode { "ON" } else { "OFF" },
                    status_style.add_modifier(Modifier::BOLD),
                ),
                Span::styled(") | ", separator_style),
                Span::styled("n", key_style),
                Span::styled("=", separator_style),
                Span::styled("cacheNotif", action_style),
                Span::styled("(", separator_style),
                Span::styled(
                    if enable_cache_hit_notification {
                        "ON"
                    } else {
                        "OFF"
                    },
                    status_style.add_modifier(Modifier::BOLD),
                ),
                Span::styled(") | ", separator_style),
                Span::styled("↑/↓", key_style),
                Span::styled("=", separator_style),
                Span::styled("scroll", action_style),
            ];
            Line::from(spans)
        }
        InputMode::Editing => {
            let spans = vec![
                Span::styled(
                    "[Editing] ",
                    base_style.fg(Color::Magenta).add_modifier(Modifier::BOLD),
                ),
                Span::styled("Enter", key_style),
                Span::styled("=", separator_style),
                Span::styled("search", action_style),
                Span::styled(" | ", separator_style),
                Span::styled("Esc", key_style),
                Span::styled("=", separator_style),
                Span::styled("stop", action_style),
                Span::styled(" | ", separator_style),
                Span::styled("↑/↓", key_style),
                Span::styled("=", separator_style),
                Span::styled("history", action_style),
                Span::styled(" | ", separator_style),
                Span::styled("<type>", Style::default().fg(Color::LightGreen)),
            ];
            Line::from(spans)
        }
    };

    Paragraph::new(help_text)
}
