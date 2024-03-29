// Package for NewsAPI requests with ureq

use serde::Deserialize;

// ------------------------------------------------------------------------------------------------------------------------------------------------------

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    // Errors for this package, with underlying errors that cause them => no need for a dynamic Error
    #[error("Failed to fetch articles")] RequestFailed(ureq::Error),
    #[error("Failed to convert response to string")] ResponseToStringFailed(std::io::Error),
    #[error("Failed to parse Articles")] ArticleParseFailed(serde_json::Error)
}

#[derive(Deserialize, Debug)]
pub struct Articles {
    pub articles: Vec<Article>
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub url: String
}

// ------------------------------------------------------------------------------------------------------------------------------------------------------

pub fn get_articles(url: &str) -> Result<Articles, NewsApiError> {
    let response: String = ureq::get(url).call().map_err(|e |NewsApiError::RequestFailed(e))?.into_string().map_err(|e |NewsApiError::ResponseToStringFailed(e))?;
    let articles: Articles = serde_json::from_str(&response).map_err(|e |NewsApiError::ArticleParseFailed(e))?;
    Ok(articles)
}
