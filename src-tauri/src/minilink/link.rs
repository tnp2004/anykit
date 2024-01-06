use crate::{validation, Error, Result};
use urlshortener::{client::UrlShortener, providers::Provider};

pub fn get_short_link(link: String) -> Result<String> {
    validation::validate_link(&link)?;

    let url_shortener = match UrlShortener::new() {
        Ok(url_shortener) => url_shortener,
        Err(_) => {
            return Err(Error::Other);
        }
    };

    let link = match url_shortener.generate(link, &Provider::IsGd) {
        Ok(link) => link,
        Err(_) => {
            return Err(Error::Generate("cannot generate short link".to_string()));
        }
    };
    println!("short link: {}", link);

    Ok(link)
}
