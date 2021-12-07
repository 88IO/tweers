use dotenv::dotenv;
use std::env;
use std::collections::HashMap;

mod twitter;

fn main() {
    dotenv().ok();

    let consummer_key = env::var("CK").expect("CK must be set.");
    let consummer_secret = env::var("CS").expect("CS must be set.");
    let access_token_key = env::var("AT").expect("AT must be set.");
    let access_token_secret = env::var("AS").expect("AS must be set.");

    //let twitter_url = env::var("TWITTER_URL").expect("TWITTER_URL must be set.");

    let twitter = twitter::Twitter::new(
        consummer_key,
        consummer_secret,
        access_token_key,
        access_token_secret
    );

    let mut params = HashMap::new();
    params.insert("status", "別の実装で投稿");

    let res = twitter.post("statuses/update", params);
    println!("{:?}", res);

    //let res = twitter.destroy_status(&res.id_str);
    //println!("{}/status/{}", twitter_url, res.id_str);
}
