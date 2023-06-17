use tweers;
use tweers::V2;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let consummer_key = env::var("CK").expect("CK must be set.");
    let consummer_secret = env::var("CS").expect("CS must be set.");
    let access_token_key = env::var("AT").expect("AT must be set.");
    let access_token_secret = env::var("AS").expect("AS must be set.");

    let twitter_url = "https://twitter.com/scienceboy_jp";

    // initialize client
    let mut twitter = tweers::Client::new(consummer_key, consummer_secret);
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
