use urlshortener::{client::UrlShortener, providers::Provider};
use crate::{Error, Result};

pub fn validate_link(link: &String) -> Result<()> {
    if link.trim().is_empty() {
        return Err(Error::InvalidInput("link cannot be empty".to_string()));
    }

    let link_pattern = r"(https:\/\/www\.|http:\/\/www\.|https:\/\/|http:\/\/)?[a-zA-Z0-9]{2,}(\.[a-zA-Z0-9]{2,})(\.[a-zA-Z0-9]{2,})?";
    let regx = regex::Regex::new(link_pattern).unwrap();

    let Some(_) = regx.captures(link.as_str()) else {
        return Err(Error::InvalidInput("link is invalid".to_string()));
    };

    Ok(())
}

pub fn get_short_link(link: String) -> Result<String> {
    validate_link(&link)?;

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

    Ok(link)
}
