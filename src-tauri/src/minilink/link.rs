use urlshortener::{client::UrlShortener, providers::Provider};

use crate::Error;
use crate::Result;

pub fn get_short_link(url: String) -> Result<String> {
    let url_shortener = match UrlShortener::new() {
        Ok(url_shortener) => url_shortener,
        Err(e) => {
            return Err(Error::Minilink(e.to_string()));
        }
    
    };

    let link = match url_shortener.generate(url, &Provider::IsGd) {
        Ok(link) => link,
        Err(_) => {
            return Err(Error::Minilink("cannot generate short link".to_string()));
        }
    };

    Ok(link)
}