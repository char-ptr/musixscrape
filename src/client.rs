use reqwest::Request;

use crate::lyrics::{LyricErrors, Lyrics};

pub struct MusixClient {
    http: reqwest::Client,
}

impl MusixClient {
    const BASE_URL: &'static str = "https://www.musixmatch.com";
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::builder().user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:109.0) Gecko/20100101 Firefox/119.0").build().unwrap(),
        }
    }
    pub async fn fetch_lyrics(&self, from_base: &str) -> Result<Lyrics, LyricErrors> {
        let url = format!("{}/{}", Self::BASE_URL, from_base);
        let req = self.http.get(url.as_str()).send().await;
        let html = req.unwrap().text().await.unwrap();
        Lyrics::try_from(html.as_str())
    }
}
