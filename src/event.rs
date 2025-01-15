use crate::app::App;
use crate::ui::{DisplayMode, InputMode};
use color_eyre::Result;
use crossterm::event::{self, Event as CEvent, KeyCode, KeyEventKind};
use std::time::Duration;

pub async fn handle_events(app: &mut App) -> Result<()> {
    if event::poll(Duration::from_millis(100))? {
        if let CEvent::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.display_mode = DisplayMode::Home;
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => app.should_quit = true,
                    KeyCode::Char('d') => app.toggle_debug_mode(),
                    KeyCode::Char('n') => app.toggle_cache_notification(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous_result(),
                    KeyCode::Down | KeyCode::Char('j') => app.next_result(),
                    KeyCode::Char('c') => app.clear_input(),
                    KeyCode::Char('o') => app.open_url(),
                    _ => {}
                },
                InputMode::Editing if key.kind == KeyEventKind::Press => {
                    app.messages.clear();
                    app.error_message = None;
                    app.has_entered = false;
                    match key.code {
                        KeyCode::Enter => {
                            app.submit().await?;
                            app.input_mode = InputMode::Normal;
                            app.has_entered = true;
                            app.history.index = 0;
                            app.history.show_history_popup = false;
                        }
                        KeyCode::Char(c) => {
                            app.insert_char(c);
                            app.history.index = 0;
                            app.history.show_history_popup = false;
                        }
                        KeyCode::Backspace => {
                            app.delete_char();
                            app.history.index = 0;
                            app.history.show_history_popup = false;
                        }
                        KeyCode::Up => {
                            if !app.history.search_history.is_empty() {
                                app.previous_history();
                                app.set_input_to_history();
                            }
                        }
                        KeyCode::Down => {
                            if app.history.show_history_popup
                                && !app.history.search_history.is_empty()
                            {
                                app.next_history();
                                app.set_input_to_history();
                            }
                        }
                        KeyCode::Left => app.move_cursor_left(),
                        KeyCode::Right => app.move_cursor_right(),
                        KeyCode::Esc => app.exit_input_mode(),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}
