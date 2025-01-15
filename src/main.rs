mod app;
mod data;
mod event;
mod search;
mod ui;
mod utils;

use color_eyre::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, Stdout};

use crate::app::App;
use crate::event::handle_events;

#[tokio::main]
async fn main() -> Result<()> {
    utils::setup_panic_hook();
    color_eyre::install()?;

    let mut terminal = setup_terminal()?;
    let mut app = App::new().await?;

    loop {
        terminal.draw(|frame| app.ui(frame))?;

        handle_events(&mut app).await?;

        if app.should_quit {
            break;
        }
    }

    restore_terminal()?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
