use color_eyre::Result;
use std::{fs, path::PathBuf};

pub struct History {
    pub search_history: Vec<String>,
    pub index: usize,
    pub show_history_popup: bool,
}

impl History {
    pub async fn load() -> Result<Self> {
        let history_path = Self::get_history_file_path().await?;
        let search_history = if history_path.exists() {
            let contents = fs::read_to_string(history_path)?;
            contents.lines().map(|s| s.trim_end().to_string()).collect()
        } else {
            Vec::new()
        };

        Ok(Self {
            search_history,
            index: 0,
            show_history_popup: false,
        })
    }

    async fn get_history_file_path() -> Result<PathBuf> {
        let data_dir = dirs::data_local_dir()
            .ok_or(color_eyre::eyre::eyre!("No local data directory found"))?;
        let app_dir = data_dir.join("terminal_google_search");
        fs::create_dir_all(&app_dir)?;
        Ok(app_dir.join("search_history.txt"))
    }

    pub async fn save(&self) -> Result<()> {
        let history_path = Self::get_history_file_path().await?;
        let mut file = fs::File::create(history_path)?;
        for query in &self.search_history {
            use std::io::Write;
            writeln!(file, "{}", query)?;
        }
        Ok(())
    }

    pub async fn add_query(&mut self, query: String) {
        if let Some(pos) = self.search_history.iter().position(|x| x == &query) {
            self.search_history.remove(pos);
        }
        self.search_history.insert(0, query.clone());
        self.index = 0;
        self.show_history_popup = false;
        let _ = self.save().await;
    }

    pub fn next(&mut self) {
        if self.search_history.is_empty() {
            return;
        }
        self.index = if self.index >= self.search_history.len() - 1 {
            0
        } else {
            self.index + 1
        };
        self.show_history_popup = true;
    }

    pub fn previous(&mut self) {
        if self.search_history.is_empty() {
            return;
        }
        self.index = if self.index == 0 {
            self.search_history.len() - 1
        } else {
            self.index - 1
        };
        self.show_history_popup = true;
    }

    pub fn get_current(&self) -> &str {
        if self.search_history.is_empty() {
            ""
        } else {
            &self.search_history[self.index]
        }
    }

    pub fn get_queries(&self) -> &[String] {
        &self.search_history
    }
}
