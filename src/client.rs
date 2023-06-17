//! ## Client
//! Twitter API Client
//!
//! This client uses OAuth 1.0a and provides REST APIs.
use base64;
use chrono::Utc;
use reqwest;
use std::collections::{HashMap, BTreeMap};
use serde_json::Value;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use percent_encoding::{utf8_percent_encode, AsciiSet};

/// Structure that holds information such as Twitter access token
pub struct Client {
    consumer_key: String,
    consumer_secret: String,
    access_token_key: Option<String>,
    access_token_secret: Option<String>,
    bearer_token: Option<String>
}

impl Client {
    /// Encoding rules for RFC3986 percent encoding
    const FRAGMENT: AsciiSet = percent_encoding::NON_ALPHANUMERIC
        .remove(b'*')
        .remove(b'-')
        .remove(b'.')
        .remove(b'_');

    /// Constructor
    ///
    /// * `consumer_key` - Comsumer key
    /// * `consumer_secret` - Comsumer secret
    pub fn new(consumer_key: String, consumer_secret: String) -> Client {
        Client {
            consumer_key,
            consumer_secret,
            access_token_key: None,
            access_token_secret: None,
            bearer_token: None
        }
    }
    
    /// set access token to use OAuth 1.0a
    /// * `access_token_key - Access token
    /// * `access_token_secret` - Access token secret
    pub fn set_access_token(&mut self, access_token_key: String, access_token_secret: String) {
        self.access_token_key = Some(access_token_key);
        self.access_token_secret = Some(access_token_secret);
    }

    /// Generating request header
    ///
    /// * `method` - HTTP method, must be uppercase
    /// * `endpoint` - request endpoint, see [here](https://developer.twitter.com/en/docs/twitter-api).
    fn generate_oauth1a_header(&self, method: &str, endpoint: &str) -> String {
        let nonce = format!("nonce{}", Utc::now().timestamp());
        let timestamp = format!("{}", Utc::now().timestamp());
        // oauth_*パラメータ
        let mut oauth_params: BTreeMap<&str, &str> = BTreeMap::new();
        oauth_params.insert("oauth_consumer_key", &self.consumer_key);
        oauth_params.insert("oauth_nonce", &nonce);
        oauth_params.insert("oauth_signature_method", "HMAC-SHA1");
        oauth_params.insert("oauth_timestamp", &timestamp);
        oauth_params.insert("oauth_token", self.access_token_key.as_ref().unwrap());
        oauth_params.insert("oauth_version", "1.0");

        // シグネチャを計算
        let oauth_signature = self.calc_oauth1a_signature(
            method, endpoint,
            &self.consumer_secret, self.access_token_secret.as_ref().unwrap(),
            &oauth_params);

        // シグネチャをoauth_*パラメータに追加
        oauth_params.insert("oauth_signature", &oauth_signature);

        // ヘッダを返す
        format!(
            "OAuth {}",
            oauth_params
                .into_iter()
                .map(|(key, value)| {
                    format!(r#"{}="{}""#,
                            utf8_percent_encode(key, &Self::FRAGMENT),
                            utf8_percent_encode(value, &Self::FRAGMENT))
                })
                .collect::<Vec<String>>()
                .join(", ")
            )
    }

    // シグネチャ生成
    fn calc_oauth1a_signature(
        &self, method: &str, endpoint: &str,
        consumer_secret: &str, access_token_secret: &str,
        params: &BTreeMap<&str, &str>
        ) -> String {

        let key: String = format!("{}&{}",
                                  utf8_percent_encode(consumer_secret, &Self::FRAGMENT),
                                  utf8_percent_encode(access_token_secret, &Self::FRAGMENT));

        let param_string = params
            .iter()
            .map(|(key, value)| {
                format!("{}={}",
                        utf8_percent_encode(key, &Self::FRAGMENT),
                        utf8_percent_encode(value, &Self::FRAGMENT))
            })
            .collect::<Vec<String>>()
            .join("&");

        let data = format!("{}&{}&{}",
                           utf8_percent_encode(method, &Self::FRAGMENT),
                           utf8_percent_encode(endpoint, &Self::FRAGMENT),
                           utf8_percent_encode(&param_string, &Self::FRAGMENT));

        let hash = hmacsha1::hmac_sha1(key.as_bytes(), data.as_bytes());

        base64::encode(&hash)
    }

    /// HTTP request.
    ///
    /// * `method` - http method
    /// * `endpoint` - request path, see [here](https://developer.twitter.com/en/docs/twitter-api).
    /// * `body` - json array of the request body
    pub async fn request(&self, method: reqwest::Method, endpoint: &str, body: HashMap<&str, &str>) -> Result<Value, reqwest::Error> {
        let header_auth = self.generate_oauth1a_header(method.as_str(), &endpoint);

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, header_auth.parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        reqwest::Client::new()
            .request(method, endpoint)
            .headers(headers)
            .json(&body)
            .send()
            .await?
            .json::<Value>()
            .await
    }

    /// HTTP GET request.
    ///
    /// * `endpoint` - request path, see [here](https://developer.twitter.com/en/docs/twitter-api).
    /// * `body` - json array of the request body
    pub async fn get(&self, endpoint: &str, body: HashMap<&str, &str>) -> Result<Value, reqwest::Error> {
        self.request(reqwest::Method::GET, endpoint, body).await
    }

    /// HTTP PUT request.
    ///
    /// * `endpoint` - request path, see [here](https://developer.twitter.com/en/docs/twitter-api).
    /// * `body` - json array of the request body
    pub async fn put(&self, endpoint: &str, body: HashMap<&str, &str>) -> Result<Value, reqwest::Error> {
        self.request(reqwest::Method::PUT, endpoint, body).await
    }

    /// HTTP POST request.
    ///
    /// * `endpoint` - request path, see [here](https://developer.twitter.com/en/docs/twitter-api).
    /// * `body` - json array of the request body
    pub async fn post(&self, endpoint: &str, body: HashMap<&str, &str>) -> Result<Value, reqwest::Error> {
        self.request(reqwest::Method::POST, endpoint, body).await
    }

    /// HTTP DELETE request.
    ///
    /// * `endpoint` - request path, see [here](https://developer.twitter.com/en/docs/twitter-api).
    /// * `body` - json array of the request body
    pub async fn delete(&self, endpoint: &str, body: HashMap<&str, &str>) -> Result<Value, reqwest::Error> {
        self.request(reqwest::Method::DELETE, endpoint, body).await
    }
}
