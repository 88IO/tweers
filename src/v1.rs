use crate::Client;
use std::collections::HashMap;
use serde_json::Value;
use reqwest::Error;
use async_trait::async_trait;

/// ## Twitter API v1.1 utility
/// V1 Trait provides API wrapper for easy handling of Twitter API v1.1.
///
/// References: [https://developer.twitter.com/en/docs/api-reference-index](https://developer.twitter.com/en/docs/api-reference-index)
///
/// | Twitter API v1.1 Endpoint      | `API` Method            |
/// | ------------------------------ | ----------------------- |
/// | POST /1.1/statuses/update.json | `Client.create_tweet()` |
/// | POST /1.1/statuses/destroy/:id | `Client.delete_tweet()` |
#[async_trait]
pub trait V1 {
    /// Create tweet
    ///
    /// * `text` - tweet content
    async fn create_tweet(&self, text: &str) -> Result<Value, Error>;

    /// Delete tweet
    ///
    /// * `id` - tweet id to delete
    async fn delete_tweet(&self, id: &str) -> Result<Value, Error>;
}


#[async_trait]
impl V1 for Client {
    async fn create_tweet(&self, text: &str) -> Result<Value, Error>
    {
        let endpoint = "https://api.twitter.com/1.1/statuses/update.json";

        let mut query = HashMap::new();
        query.insert("status", text);

        self.post(endpoint)
            .query(&query)
            .send()
            .await?
            .json::<Value>()
            .await
    }

    async fn delete_tweet(&self, id: &str) -> Result<Value, Error>
    {
        let endpoint = format!("https://api.twitter.com/1.1/statuses/destroy/{}.json", id);

        self.post(&endpoint)
            .send()
            .await?
            .json::<Value>()
            .await
    }
}
