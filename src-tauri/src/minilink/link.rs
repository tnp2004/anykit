use urlshortener::{client::UrlShortener, providers::Provider};

use crate::Result;

pub fn get_short_link(url: String) -> String {
    let url_shortener = UrlShortener::new().unwrap();
    let link = url_shortener.generate(url, &Provider::IsGd).unwrap();

    link
}