# tweers

[![Version](https://img.shields.io/badge/version-0.1.0-orange)](https://88IO.github.io/tweers/tweers/)
[![Rust](https://github.com/88IO/tweers/actions/workflows/rust.yml/badge.svg)](https://github.com/88IO/tweers/actions/workflows/rust.yml)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

An easy-to-use Rust library for accessing the Twitter API.

This library uses Twitter API v2 and OAuth 1.0a and provides REST APIs.

## Features
- **OAuth 1.0a** : Included by default. See the examples for how to use.
- **Twitter API v2 wrapper** : Provide APIs inspired `tweepy`
- **rust-tls** : Use `rustls` as TLS backend

## Reference
[https://88IO.github.io/tweers/tweers/](https://88IO.github.io/tweers/tweers/)

## Example
This asynchronous example uses [Tokio](https://tokio.rs/) and [dotenv](https://crates.io/crates/dotenv),
so your `Cargo.toml` could look like this:

```
[dev-dependencies]
dotenv = "0.15.0"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

And then `.env`:

- CK: consumer key
- CS: consumer secret
- AT: access token
- AS: access token secret

```
CK=XXXXXXXXXXXXXXXXXXXXXXXXX
CS=XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
AT=XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
AS=XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

create and delete tweet example:

```rust
use tweers;
use tweers::V2;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let consumer_key = env::var("CK").expect("CK must be set.");
    let consumer_secret = env::var("CS").expect("CS must be set.");
    let access_token_key = env::var("AT").expect("AT must be set.");
    let access_token_secret = env::var("AS").expect("AS must be set.");

    let twitter_url = "https://twitter.com/scienceboy_jp";

    // initialize client
    let mut twitter = tweers::Client::new(consumer_key, consumer_secret);
    twitter.set_access_token(access_token_key, access_token_secret);

    // create tweet
    let res = twitter.create_tweet("test").await.unwrap();
    println!("{:?}", res);

    let id = res["data"]["id"].as_str().unwrap();
    println!("{}/status/{}", twitter_url, id);

    // delete tweet
    let res = twitter.delete_tweet(id).await.unwrap();
    println!("{:?}", res);
}
```

## License
Licensed under either of

- Apache License, Version 2.0 (LICENSE or http://apache.org/licenses/LICENSE-2.0)

## Blog
- [https://scienceboy.jp/88io/2022/01/rust-tweet/](https://scienceboy.jp/88io/2022/01/rust-tweet/)
