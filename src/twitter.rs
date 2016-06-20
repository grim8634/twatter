use oauth_client::Token;
use twitter_api::Tweet;
use twitter_api;
use counter;
use std::time::Duration;
use std::thread;
use config::TwatterConfig;

pub fn run(config: &TwatterConfig) {
    let conf = &config.twitter;

    let consumer = Token::new(conf.consumer_key.to_string(), conf.consumer_secret.to_string());
    let access = Token::new(conf.access_key.to_string(), conf.access_secret.to_string());

    loop {
        println!("Checking for tweets....");
        let max_id = counter::get() as u64;
        let mut new_max_id = max_id;

        let tweets = get_tweets(&consumer, &access);
        if ! tweets.is_none() {
            let tweets = tweets.unwrap();
            if tweets.is_empty() {
                println!("No tweets in the timeline... tweet something already!");
            } else {
                for tweet in tweets {
                    if tweet.id > max_id {
                        if tweet.id > new_max_id {
                            new_max_id = tweet.id;
                        }
                        process_tweet(tweet, &consumer, &access, &config);
                    }
                }
            }
        }

        counter::set(new_max_id).unwrap();
        thread::sleep(Duration::from_secs(60)); //Run every 60 seconds...
    }
}

fn process_tweet(tweet:Tweet, consumer:&Token, access:&Token, config: &TwatterConfig) {
    if tweet.user.screen_name != "1hgscouts" {
        retweet(tweet, &consumer, &access, &config);
    }
}

fn retweet(tweet:Tweet, consumer:&Token, access:&Token, config: &TwatterConfig) {
    println!("{:?}", tweet);
    let new_message = add_user_initials(&tweet, &config);

    if new_message.len() > 140 {
        println!("\tTWEET TOO LONG");
        match twitter_api::direct_message(&consumer, &access, "Your message was too long and wasn't retweeted", &tweet.user.screen_name) {
            Err(e) => println!("Failed to send DM, are they following us? {}", e),
            Ok(v) => v
        }
    } else {
        println!("{}: {}", tweet.user.screen_name, &new_message);
        match twitter_api::update_status(&consumer, &access, &new_message) {
            Err(e) => println!("Failed to tweet message: {}", e),
            Ok(v) => v
        }
    }
}

fn add_user_initials( tweet: &Tweet, config: &TwatterConfig ) -> String {
    format!("{} ({})", &tweet.text, config.aliases[&tweet.user.screen_name].as_str().unwrap())
}

fn get_tweets(consumer:&Token, access:&Token)->Option<Vec<Tweet>> {
    match twitter_api::get_last_tweets(&consumer, &access) {
        Err(e) => {
            println!("{:?}", e);
            None
        },
        Ok(tweets) => Some(tweets),
    }

}



