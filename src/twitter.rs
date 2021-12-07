use reqwest;
use base64;
use chrono::Utc;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use percent_encoding::{utf8_percent_encode, AsciiSet};
use std::collections::HashMap;
use serde_derive::Deserialize;
use serde_json::Value;

// レスポンスで必要な部分だけ記述
// これを戻り値にせずserde_json::Valueで全部取得してもよい
#[derive(Deserialize, Debug)]
struct Response {
    id_str: String
}

// Twitterの認証関連と一部ラッパー実装
pub struct Twitter {
    consummer_key: String,
    consummer_secret: String,
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
        consummer_key: String, consummer_secret: String,
        access_token_key: String, access_token_secret: String)
        -> Twitter {
        Twitter {
            consummer_key: consummer_key,
            consummer_secret: consummer_secret,
            access_token_key: access_token_key,
            access_token_secret: access_token_secret
        }
    }

    // ヘッダー生成
    fn get_request_header(&self, method: &str, endpoint: &str, params: &HashMap<&str, &str>) -> String {

        let nonce = format!("nonce{}", Utc::now().timestamp());
        let timestamp = format!("{}", Utc::now().timestamp());
        // oauth_*パラメータ
        let mut oauth_params: HashMap<&str, &str> = HashMap::new();
        oauth_params.insert("oauth_consumer_key", &self.consummer_key);
        oauth_params.insert("oauth_nonce", &nonce);
        oauth_params.insert("oauth_signature_method", "HMAC-SHA1");
        oauth_params.insert("oauth_timestamp", &timestamp);
        oauth_params.insert("oauth_token", &self.access_token_key);
        oauth_params.insert("oauth_version", "1.0");

        // 他パラメータと結合
        let mut all_params: HashMap<&str, &str> = HashMap::new();
        all_params.clone_from(&oauth_params);
        all_params.extend(params.into_iter());

        // シグネチャを計算
        let oauth_signature: &str = &self.get_oauth_signature(
            method, endpoint,
            &self.consummer_secret, &self.access_token_secret,
            &all_params);

        // シグネチャをoauth_*パラメータに追加
        oauth_params.insert("oauth_signature", oauth_signature);

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
        consummer_secret: &str, access_token_secret: &str,
        params: &HashMap<&str, &str>
        ) -> String {

        let key: String = format!("{}&{}",
                                  utf8_percent_encode(consummer_secret, &Self::FRAGMENT),
                                  utf8_percent_encode(access_token_secret, &Self::FRAGMENT));

        let mut params: Vec<(&&str, &&str)> = params.into_iter().collect();
        params.sort();

        let param = params
            .into_iter()
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
                           utf8_percent_encode(&param, &Self::FRAGMENT));

        let hash = hmacsha1::hmac_sha1(key.as_bytes(), data.as_bytes());

        base64::encode(&hash)
    }

    pub fn post(&self, path: &str, params: HashMap<&str, &str>) -> Value {
        let endpoint = &format!("https://api.twitter.com/1.1/{}.json", path);

        let header_auth = self.get_request_header("POST", &endpoint, &params);

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, header_auth.parse().unwrap());

        reqwest::Client::new()
            .post(endpoint)
            .query(&params)
            .headers(headers)
            .send()
            .unwrap()
            .json::<Value>()
            .unwrap()
    }

    // ツイート
    pub fn update_status(&self, status: &str) -> Value  {
        let mut headers = HeaderMap::new();

        let endpoint = "https://api.twitter.com/1.1/statuses/update.json";

        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("status", status);

        let header_auth = self.get_request_header("POST", &endpoint, &params);

        headers.insert(AUTHORIZATION, header_auth.parse().unwrap());

        let client = reqwest::Client::new();

        client
            .post(endpoint)
            .query(&params)
            .headers(headers)
            .send()
            .unwrap()
            .json::<Value>()
            .unwrap()
    }

    // ツイート削除
    pub fn destroy_status(&self, id: &str) -> Response {
        let endpoint = &format!("https://api.twitter.com/1.1/statuses/destroy/{}.json", id);

        let params: HashMap<&str, &str> = HashMap::new();

        let header_auth = self.get_request_header("POST", &endpoint, &params);

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, header_auth.parse().unwrap());

        let client = reqwest::Client::new();

        client
            .post(endpoint)
            .headers(headers)
            .send()
            .unwrap()
            .json::<Response>()
            .unwrap()
    }
}
