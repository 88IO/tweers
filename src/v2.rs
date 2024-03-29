use crate::Client;
use std::collections::HashMap;
use serde_json::Value;
use reqwest::Error;
use async_trait::async_trait;

/// ## Twitter API v2 utility
/// V2 Trait provides API wrapper for easy handling of Twitter API v2.
///
/// References: [https://developer.twitter.com/en/docs/api-reference-index](https://developer.twitter.com/en/docs/api-reference-index)
///
/// | Twitter API v2 Endpoint | `API` Method            |
/// | ----------------------- | ----------------------- |
/// | POST /2/tweets          | `Client.create_tweet()` |
/// | DELETE /2/tweets        | `Client.delete_tweet()` |
#[async_trait]
pub trait V2 {
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
impl V2 for Client {
    async fn create_tweet(&self, text: &str) -> Result<Value, Error>
    {
        let endpoint = "https://api.twitter.com/2/tweets";
        let mut body = HashMap::new();
        body.insert("text", text);

        self.post(endpoint)
            .json(&body)
            .send()
            .await?
            .json::<Value>()
            .await
    }

    async fn delete_tweet(&self, id: &str) -> Result<Value, Error>
    {
        let endpoint = format!("https://api.twitter.com/2/tweets/{}", id);

        self.delete(&endpoint)
            .send()
            .await?
            .json::<Value>()
            .await
    }
}
