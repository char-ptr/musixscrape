use std::{str::FromStr, sync::Arc};

use chrono::NaiveDate;
use scraper::Selector;
use thiserror::Error;
#[derive(Debug)]
pub struct Lyrics {
    pub title: Arc<str>,
    pub artist: Arc<str>,
    pub album: Arc<str>,
    pub lyrics: Arc<str>,
    pub release_date: chrono::NaiveDate,
}
struct LyricSelectors {
    title: Selector,
    artist: Selector,
    lyrics: Selector,
    date: Selector,
    album: Selector,
}
impl Default for LyricSelectors {
    fn default() -> Self {
        Self {
            title: Selector::parse(".mxm-track-title__track").unwrap(),
            artist: Selector::parse(".mxm-track-title__artist").unwrap(),
            lyrics: Selector::parse(".lyrics__content__ok").unwrap(),
            date: Selector::parse(".mui-cell__subtitle").unwrap(),
            album: Selector::parse("a.mui-cell--sm > div:nth-child(2) > h2:nth-child(1)").unwrap(),
        }
    }
}
fn parse_shitty_date(d: &str) -> Option<NaiveDate> {
    let mut parts = d.split_whitespace();
    let month = parts.next()?;
    let day: String = parts
        .next()?
        .chars()
        .take_while(|f| f.is_numeric())
        .collect();
    let year = parts.next()?;
    NaiveDate::parse_from_str(&format!("{year}-{month}-{day:0>2}"), "%Y-%h-%d")
        // .inspect_err(|x| println!("date parse failure {:?}", x))
        .ok()
}

impl TryFrom<&str> for Lyrics {
    type Error = LyricErrors;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let selectors = LyricSelectors::default();
        let document = scraper::Html::parse_document(value);
        let mut matches = document.select(&selectors.title);
        let title_text = matches
            .next()
            .ok_or(LyricErrors::TitleNotFound)?
            .text()
            .nth(1)
            .ok_or(LyricErrors::TitleNotFound)?;
        let mut matches = document.select(&selectors.artist);
        let artist_text = matches
            .next()
            .ok_or(LyricErrors::AuthorNotFound)?
            .text()
            .nth(0)
            .ok_or(LyricErrors::AuthorNotFound)?;
        let mut matches = document.select(&selectors.album);
        let album_text = matches
            .next()
            .ok_or(LyricErrors::AlbumNotFound)?
            .text()
            .nth(0)
            .ok_or(LyricErrors::AlbumNotFound)?;
        let mut matches = document.select(&selectors.date);
        let date_text = matches
            .next()
            .ok_or(LyricErrors::DateNotFound)?
            .text()
            .nth(0)
            .ok_or(LyricErrors::DateNotFound)?;
        let date_rel = parse_shitty_date(date_text).ok_or(LyricErrors::DateParseError)?;
        let matches = document.select(&selectors.lyrics);
        let lyrics_text: String = matches
            .flat_map(|f| f.text().nth(0).ok_or(|| LyricErrors::LyricsNotFound))
            .collect();
        Ok(Lyrics {
            album: album_text.into(),
            artist: artist_text.into(),
            title: title_text.into(),
            lyrics: lyrics_text.into(),
            release_date: date_rel,
        })
    }
}
impl FromStr for Lyrics {
    type Err = LyricErrors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Lyrics::try_from(s)
    }
}
#[derive(Debug, Error)]
pub enum LyricErrors {
    #[error("set yourself on fire")]
    Unknown,
    #[error("title not found")]
    TitleNotFound,
    #[error("author not found")]
    AuthorNotFound,
    #[error("lyrics not found")]
    LyricsNotFound,
    #[error("date not found")]
    DateNotFound,
    #[error("unable to parse date")]
    DateParseError,
    #[error("album not found")]
    AlbumNotFound,
}
