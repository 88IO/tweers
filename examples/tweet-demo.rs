use tweers::Twitter;
use dotenv::dotenv;
use std::env;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let consummer_key = env::var("CK").expect("CK must be set.");
    let consummer_secret = env::var("CS").expect("CS must be set.");
    let access_token_key = env::var("AT").expect("AT must be set.");
    let access_token_secret = env::var("AS").expect("AS must be set.");

    let twitter_url = env::var("TWITTER_URL").expect("TWITTER_URL must be set.");

    let twitter = Twitter::new(
        consummer_key,
        consummer_secret,
        access_token_key,
        access_token_secret
    );

    let mut params = HashMap::new();
    params.insert("text", "Twitter API v2対応");

    let res = twitter.post("tweets", params).await.unwrap();
    println!("{:?}", res);

    let id = res["data"]["id"].as_str().unwrap();
    println!("{}/status/{}", twitter_url, id);

    let res = twitter.delete(&format!("tweets/{}", id), HashMap::new()).await.unwrap();
    println!("{:?}", res);

    //let res = twitter.post(&format!("statuses/destory/{}", res["id"].as_str().unwrap()), HashMap::new());
    //println!("{:?}", res);

    //let res = twitter.destroy_status(&res.id_str);
    //println!("{}/status/{}", twitter_url, res.id_str);
}
