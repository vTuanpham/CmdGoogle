use color_eyre::Result;
use rand::seq::SliceRandom;
use rand::thread_rng;
use scraper::{Html, Selector};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::fmt;
use std::fs;
use std::time::Duration;
use tokio::time::Instant;
use urlencoding::encode;

#[derive(Debug)]
pub struct QueryArgs {
    pub query: String,
    pub debug_mode: bool,
}

#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub api_key: String,
    pub cse_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub url: String,
    pub description: String,
    pub url_supported_flag: bool,
}

impl fmt::Display for QueryResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Url: {}\n Description: {}\n Crawl supported: {}",
            self.url, self.description, self.url_supported_flag
        )
    }
}

fn load_api_config() -> Option<ApiConfig> {
    let api_key = env::var("GOOGLE_API_KEY").ok()?;
    let cse_id = env::var("GOOGLE_CSE_ID").ok()?;
    if api_key.is_empty() || cse_id.is_empty() {
        None
    } else {
        Some(ApiConfig { api_key, cse_id })
    }
}

fn random_proxy() -> Option<reqwest::Proxy> {
    let pool = env::var("PROXY_POOL").ok()?;
    let proxies: Vec<_> = pool
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    let proxy_url = proxies.choose(&mut thread_rng())?;
    reqwest::Proxy::all(*proxy_url).ok()
}

pub fn build_http_client() -> Result<reqwest::Client> {
    let mut builder = reqwest::Client::builder();
    if let Some(proxy) = random_proxy() {
        builder = builder.proxy(proxy);
    }
    Ok(builder.build()?)
}

pub async fn search_query(args: QueryArgs) -> Result<Vec<QueryResult>> {
    let start = Instant::now();
    let results = if let Some(cfg) = load_api_config() {
        search_query_api(&args, &cfg).await?
    } else {
        search_query_scrape(&args).await?
    };
    println!("Search query took {:?}", start.elapsed());
    Ok(results)
}

async fn search_query_api(args: &QueryArgs, cfg: &ApiConfig) -> Result<Vec<QueryResult>> {
    let url = format!(
        "https://www.googleapis.com/customsearch/v1?key={}&cx={}&q={}",
        cfg.api_key,
        cfg.cse_id,
        encode(&args.query)
    );
    let resp = reqwest::get(&url).await?;
    if !resp.status().is_success() {
        return Err(color_eyre::Report::msg(format!(
            "HTTP Error: {} - {}",
            resp.status().as_u16(),
            resp.text().await?
        )));
    }
    let body: Value = resp.json().await?;
    if args.debug_mode {
        let mut file = fs::File::create("debug_raw.json")?;
        std::io::Write::write_all(&mut file, serde_json::to_string_pretty(&body)?.as_bytes())?;
    }
    let items = body["items"].as_array().cloned().unwrap_or_default();
    let results = items
        .into_iter()
        .filter_map(|item| {
            let url = item["link"].as_str()?.to_string();
            let snippet = item["snippet"].as_str().unwrap_or("").to_string();
            Some(QueryResult {
                url,
                description: snippet,
                url_supported_flag: true,
            })
        })
        .collect();
    Ok(results)
}

async fn search_query_scrape(args: &QueryArgs) -> Result<Vec<QueryResult>> {
    let search_url = format!("https://www.google.com/search?q={}", encode(&args.query));
    let user_agents = [
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.138 Safari/537.36",
        "Mozilla/5.0 (Windows NT 11.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
    ];
    let random_user_agent = user_agents.choose(&mut thread_rng()).unwrap();
    let client = build_http_client()?;
    let resp = client
        .get(&search_url)
        .header("User-Agent", *random_user_agent)
        .header("Accept-Language", "en-US,en;q=0.9")
        .header("Referer", "https://www.google.com/")
        .timeout(Duration::from_secs(10))
        .send()
        .await?;
    if !resp.status().is_success() {
        return Err(color_eyre::Report::msg(format!(
            "HTTP Error: {} - {}",
            resp.status().as_u16(),
            resp.text().await?,
        )));
    }
    let body = resp.text().await?;
    if args.debug_mode {
        let mut file = fs::File::create("debug_raw.html")?;
        std::io::Write::write_all(&mut file, body.as_bytes())?;
    }
    parse_search_results(&body)
}

fn parse_search_results(html: &str) -> Result<Vec<QueryResult>> {
    let doc = Html::parse_document(html);
    let container_sel = parse_selector(r#"div[class=\"MjjYud\"]"#)?;
    let link_sel = parse_selector(r#"a[jsname=\"UWckNb\"]"#)?;
    let desc_sel = parse_selector(r#"div[class=\"kb0PBd A9Y9g\"]"#)?;
    let mut results = Vec::new();
    for container in doc.select(&container_sel) {
        let url = container
            .select(&link_sel)
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(String::from);
        let desc = container
            .select(&desc_sel)
            .next()
            .map(|d| d.text().collect::<String>().trim().to_string());
        if let (Some(url), Some(desc)) = (url, desc) {
            results.push(QueryResult {
                url,
                description: desc,
                url_supported_flag: true,
            });
        }
    }
    Ok(results)
}

fn parse_selector(selector: &str) -> Result<Selector> {
    Selector::parse(selector)
        .map_err(|e| color_eyre::Report::msg(format!("Selector parse error: {:?}", e)))
}
