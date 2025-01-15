use crate::search::QueryResult;
use color_eyre::Result;
use serde_derive::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Serialize, Deserialize)]
struct CachedResult {
    results: Vec<QueryResult>,
    timestamp: u64,
}

pub struct Cache {
    data: HashMap<String, CachedResult>,
    pub cache_hit: bool,
    pub enable_cache_hit_notification: bool,
}

impl Cache {
    pub async fn load() -> Result<Self> {
        let cache_path = Self::get_cache_file_path().await?;
        let data = if cache_path.exists() {
            let file = fs::File::open(cache_path)?;
            bincode::deserialize_from(file).unwrap_or_default()
        } else {
            HashMap::new()
        };
        Ok(Self {
            data,
            cache_hit: false,
            enable_cache_hit_notification: true,
        })
    }

    async fn get_cache_file_path() -> Result<PathBuf> {
        let cache_dir =
            dirs::cache_dir().ok_or(color_eyre::eyre::eyre!("No cache directory found"))?;
        let app_cache_dir = cache_dir.join("terminal_google_search");
        fs::create_dir_all(&app_cache_dir)?;
        Ok(app_cache_dir.join("search_cache.bin"))
    }

    pub async fn save(&self) -> Result<()> {
        let cache_path = Self::get_cache_file_path().await?;
        let file = fs::File::create(cache_path)?;
        bincode::serialize_into(file, &self.data)?;
        Ok(())
    }

    pub async fn get(&mut self, query: &str) -> Result<Option<Vec<QueryResult>>> {
        if let Some(cached) = self.data.get(query) {
            let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
            if now - cached.timestamp < Duration::from_secs(60 * 60 * 24).as_secs() {
                return Ok(Some(cached.results.clone()));
            } else {
                self.data.remove(query);
                self.save().await?;
            }
        }
        Ok(None)
    }

    pub async fn insert(&mut self, query: String, results: Vec<QueryResult>) -> Result<()> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        self.data.insert(query, CachedResult { results, timestamp });
        self.save().await?;
        Ok(())
    }
}
