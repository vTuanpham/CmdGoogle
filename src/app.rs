use crate::data::{cache::Cache, history::History};
use crate::search::{search_query, QueryArgs, QueryResult};
use crate::ui::{components::*, home, ready, searching, DisplayMode, InputMode};
use crate::utils::StringExt;
use color_eyre::Result;
use ratatui::{widgets::ListState, Frame};
use std::time::Duration;

pub struct App {
    pub input: String,
    pub cursor_idx: usize,
    pub input_mode: InputMode,
    pub messages: Vec<QueryResult>,
    pub selected_idx: usize,
    pub debug_mode: bool,
    pub has_entered: bool,
    pub is_loading: bool,
    pub error_message: Option<String>,
    pub display_mode: DisplayMode,
    pub spinner_frames: Vec<char>,
    pub spinner_index: usize,
    pub results_list_state: ListState,
    pub history_list_state: ListState,
    pub should_quit: bool,
    pub cache: Cache,
    pub history: History,
}

impl App {
    pub async fn new() -> Result<Self> {
        let history = History::load().await?;
        let cache = Cache::load().await?;

        Ok(Self {
            input: String::new(),
            cursor_idx: 0,
            input_mode: InputMode::Editing,
            messages: Vec::new(),
            selected_idx: 0,
            debug_mode: false,
            has_entered: false,
            is_loading: false,
            error_message: None,
            display_mode: DisplayMode::Home,
            spinner_frames: vec!['|', '/', '-', '\\'],
            spinner_index: 0,
            results_list_state: ListState::default(),
            history_list_state: ListState::default(),
            should_quit: false,
            cache,
            history,
        })
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_idx > 0 {
            self.cursor_idx -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_idx < self.input.chars().count() {
            self.cursor_idx += 1;
        }
    }

    pub fn insert_char(&mut self, c: char) {
        self.cache.cache_hit = false;
        let byte_idx = self
            .input
            .char_indices()
            .nth(self.cursor_idx)
            .map(|(i, _)| i)
            .unwrap_or(self.input.len());
        self.input.insert(byte_idx, c);
        self.cursor_idx += 1;
    }

    pub fn delete_char(&mut self) {
        self.cache.cache_hit = false;
        if self.cursor_idx > 0 {
            let byte_idx = self
                .input
                .char_indices()
                .nth(self.cursor_idx - 1)
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.input.remove(byte_idx);
            self.cursor_idx -= 1;
        }
    }

    pub fn next_result(&mut self) {
        if self.messages.is_empty() {
            return;
        }
        let i = match self.results_list_state.selected() {
            Some(i) => {
                if i >= self.messages.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.results_list_state.select(Some(i));
        self.selected_idx = i;
    }

    pub fn previous_result(&mut self) {
        if self.messages.is_empty() {
            return;
        }
        let i = match self.results_list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.messages.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.results_list_state.select(Some(i));
        self.selected_idx = i;
    }

    pub async fn submit(&mut self) -> Result<()> {
        let query = self.input.trim().to_lowercase();
        if query.is_empty() {
            return Ok(());
        }

        self.history.add_query(query.clone()).await;

        self.display_mode = DisplayMode::Searching;
        self.is_loading = true;

        if let Some(cached_results) = self.cache.get(&query).await? {
            tokio::time::sleep(Duration::from_millis(600)).await;
            self.messages = cached_results;
            self.error_message = None;
            self.is_loading = false;
            self.cache.cache_hit = true;
            self.display_mode = DisplayMode::Ready;
            return Ok(());
        }

        self.cache.cache_hit = false;
        self.messages.clear();
        self.error_message = None;

        let search_result = search_query(QueryArgs {
            query: query.clone(),
            debug_mode: self.debug_mode,
        })
        .await;

        match search_result {
            Ok(results) => {
                tokio::time::sleep(Duration::from_millis(600)).await;
                self.messages = results.clone();
                self.cache.insert(query.clone(), results).await?;
            }
            Err(e) => {
                self.error_message = Some(format!("{}", e));
            }
        }

        self.is_loading = false;
        self.cache.cache_hit = false;
        self.display_mode = DisplayMode::Ready;
        Ok(())
    }

    pub fn next_history(&mut self) {
        self.history.next();
        self.history_list_state.select(Some(self.history.index));
    }

    pub fn previous_history(&mut self) {
        self.history.previous();
        self.history_list_state.select(Some(self.history.index));
    }

    pub fn set_input_to_history(&mut self) {
        self.input = self.history.get_current().trim_end().to_string();
        self.cursor_idx = self.input.grapheme_len();
    }

    pub fn ui(&mut self, frame: &mut Frame) {
        let screen = frame.area();
        match self.display_mode {
            DisplayMode::Home => {
                home::render(self, frame);
                if self.history.show_history_popup {
                    let popup = create_history_popup(
                        self.history.get_queries(),
                        self.history.index,
                        screen.height as usize,
                    );
                    frame.render_widget(popup.clone(), popup.calculate_area(screen));
                }
            }
            DisplayMode::Searching => {
                if self.is_loading {
                    self.spinner_index = (self.spinner_index + 1) % self.spinner_frames.len();
                }
                searching::render(self, frame)
            }
            DisplayMode::Ready => {
                ready::render(self, frame);
                if let Some(err_msg) = &self.error_message {
                    let popup = create_error_popup(err_msg);
                    frame.render_widget(popup.clone(), popup.calculate_area(screen));
                } else if self.messages.is_empty() && self.has_entered && !self.is_loading {
                    let popup = create_warning_popup(&self.input);
                    frame.render_widget(popup.clone(), popup.calculate_area(screen));
                }

                if self.cache.cache_hit && self.cache.enable_cache_hit_notification {
                    let popup = create_cache_popup();
                    frame.render_widget(popup.clone(), popup.calculate_top_right_area(screen));
                }
            }
        }
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
        self.cursor_idx = 0;
        self.cache.cache_hit = false;
        self.has_entered = false;
        self.display_mode = DisplayMode::Home;
        self.input_mode = InputMode::Editing;
    }

    pub fn open_url(&mut self) {
        if let Some(message) = self.messages.get(self.selected_idx) {
            if let Err(e) = open::that(&message.url) {
                self.error_message = Some(format!("Error opening URL: {}", e));
            }
        }
    }

    pub fn toggle_debug_mode(&mut self) {
        self.debug_mode = !self.debug_mode;
    }

    pub fn toggle_cache_notification(&mut self) {
        self.cache.enable_cache_hit_notification = !self.cache.enable_cache_hit_notification;
    }

    pub fn exit_input_mode(&mut self) {
        self.input_mode = InputMode::Normal;
        self.history.show_history_popup = false;
    }
}
