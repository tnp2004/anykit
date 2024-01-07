use crate::{validation, Error, Result};
use urlshortener::{client::UrlShortener, providers::Provider};

pub async fn get_short_link(link: String) -> Result<String> {
    validation::validate_link(&link)?;

    let url_shortener = match tokio::task::spawn_blocking(|| UrlShortener::new()).await {
        Ok(Ok(url_shortener)) => url_shortener,
        Ok(Err(_)) | Err(_) => {
            return Err(Error::Other);
        }
    };

    let link = match tokio::task::spawn_blocking(move || url_shortener.generate(link, &Provider::IsGd)).await {
        Ok(Ok(link)) => link,
        Ok(Err(_)) | Err(_) => {
            return Err(Error::Generate("cannot generate short link".to_string()));
        }
    };

    Ok(link)
}