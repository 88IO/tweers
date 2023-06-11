use base64;
use chrono::Utc;
use reqwest;
use std::collections::BTreeMap;
use serde_json::Value;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use percent_encoding::{utf8_percent_encode, AsciiSet};

// Twitterの認証関連と一部ラッパー実装
pub struct Twitter {
    consumer_key: String,
    consumer_secret: String,
    access_token_key: String,
    access_token_secret: String
}

impl Twitter {
    // エンコードルール
    const FRAGMENT: AsciiSet = percent_encoding::NON_ALPHANUMERIC
        .remove(b'*')
        .remove(b'-')
        .remove(b'.')
        .remove(b'_');

    // インスタンス生成
    pub fn new(
        consumer_key: String, consumer_secret: String,
        access_token_key: String, access_token_secret: String)
        -> Twitter {
        Twitter {
            consumer_key,
            consumer_secret,
            access_token_key,
            access_token_secret
        }
    }

    // ヘッダー生成
    fn get_request_header(&self, method: &str, endpoint: &str) -> String {
        let nonce = format!("nonce{}", Utc::now().timestamp());
        let timestamp = format!("{}", Utc::now().timestamp());
        // oauth_*パラメータ
        let mut oauth_params: BTreeMap<&str, &str> = BTreeMap::new();
        oauth_params.insert("oauth_consumer_key", &self.consumer_key);
        oauth_params.insert("oauth_nonce", &nonce);
        oauth_params.insert("oauth_signature_method", "HMAC-SHA1");
        oauth_params.insert("oauth_timestamp", &timestamp);
        oauth_params.insert("oauth_token", &self.access_token_key);
        oauth_params.insert("oauth_version", "1.0");

        // シグネチャを計算
        let oauth_signature = self.get_oauth_signature(
            method, endpoint,
            &self.consumer_secret, &self.access_token_secret,
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
    fn get_oauth_signature(
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

    pub async fn get(&self, path: &str, params: HashMap<&str, &str>) -> Result<Value, reqwest::Error> {
        let endpoint = format!("https://api.twitter.com/1.1/{}.json", path);

        let header_auth = self.get_request_header("GET", &endpoint);

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, header_auth.parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        reqwest::Client::new()
            .get(&endpoint)
            .headers(headers)
            .json(&params)
            .send()
            .await?
            .json::<Value>()
            .await
    }

    pub async fn put(&self, path: &str, params: HashMap<&str, &str>) -> Result<Value, reqwest::Error> {
        let endpoint = format!("https://api.twitter.com/1.1/{}.json", path);

        let header_auth = self.get_request_header("PUT", &endpoint);

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, header_auth.parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        reqwest::Client::new()
            .put(&endpoint)
            .headers(headers)
            .json(&params)
            .send()
            .await?
            .json::<Value>()
            .await
    }

    pub async fn post(&self, path: &str, params: HashMap<&str, &str>) -> Result<Value, reqwest::Error> {
        let endpoint = format!("https://api.twitter.com/2/{}", path);

        let header_auth = self.get_request_header("POST", &endpoint);

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, header_auth.parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        reqwest::Client::new()
            .post(&endpoint)
            .headers(headers)
            .json(&params)
            .send()
            .await?
            .json::<Value>()
            .await
    }

    pub async fn delete(&self, path: &str, params: HashMap<&str, &str>) -> Result<Value, reqwest::Error> {
        let endpoint = format!("https://api.twitter.com/2/{}", path);

        let header_auth = self.get_request_header("DELETE", &endpoint);

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, header_auth.parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        reqwest::Client::new()
            .delete(&endpoint)
            .headers(headers)
            .json(&params)
            .send()
            .await?
            .json::<Value>()
            .await
    }
}
