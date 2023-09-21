use thiserror::Error;

use crate::lyrics::{LyricErrors, Lyrics};
#[derive(Debug, Error)]
pub enum MusixClientErrors {
    #[error("request error {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("lyric error {0}")]
    LyricError(#[from] LyricErrors),
}
pub type Result<T> = std::result::Result<T, MusixClientErrors>;

pub struct MusixClient {
    http: reqwest::Client,
}
impl Default for MusixClient {
    fn default() -> Self {
        Self::new()
    }
}

impl MusixClient {
    const BASE_URL: &'static str = "https://www.musixmatch.com";
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::builder().user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:109.0) Gecko/20100101 Firefox/119.0").build().unwrap(),
        }
    }
    pub async fn fetch_lyrics(&self, from_base: &str) -> Result<Lyrics> {
        let url = format!("{}/{}", Self::BASE_URL, from_base);
        let req = self.http.get(url.as_str()).send().await;
        let html = req?.text().await?;
        Lyrics::try_from(html.as_str()).map_err(MusixClientErrors::from)
    }
    pub async fn fetch_lyrics_ta(&self, title: &str, artist: &str) -> Result<Lyrics> {
        let url = format!(
            "lyrics/{}/{}",
            artist.replace(' ', "-"),
            title.replace(' ', "-"),
        );
        self.fetch_lyrics(url.as_str()).await
    }
}
