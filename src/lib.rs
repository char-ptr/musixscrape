// #![feature(result_option_inspect)]
pub mod client;
pub mod lyrics;

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use tokio::runtime::Runtime;

    use crate::{client::MusixClient, lyrics::Lyrics};

    #[test]
    fn test_lyrc() {
        Runtime::new().unwrap().block_on(async {
            let req =
                reqwest::get("https://www.musixmatch.com/lyrics/In-Flames/Only-for-the-Weak").await;
            let html = req.unwrap().text().await.unwrap();
            let lyrc = Lyrics::try_from(html.as_str());
            // println!("{:?}", lyrc);
            assert!(lyrc.is_ok());
            assert_eq!(lyrc.unwrap().title.deref(), "Only For The Weak")
        })
    }
    #[tokio::test]
    async fn fetch_lyrics() {
        let cnt = MusixClient::new();
        let lyrc = cnt
            .fetch_lyrics("/lyrics/In-Flames/Only-for-the-Weak")
            .await;
        assert!(lyrc.is_ok());
        assert_eq!(lyrc.unwrap().title.deref(), "Only For The Weak")
    }
    #[tokio::test]
    async fn fetch_lyrics_ta() {
        let cnt = MusixClient::new();
        let lyrc = cnt.fetch_lyrics_ta("Only for the Weak", "In Flames").await;
        println!("{:?}", lyrc);
        assert!(lyrc.is_ok());
        assert_eq!(lyrc.unwrap().title.deref(), "Only For The Weak")
    }
}
