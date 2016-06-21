use oauth_client::Token;
use twitter_api::{Tweet, DirectMessage};
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
        let max_id = counter::get("status.id") as u64;
        let mut new_max_id = max_id;
        let mut dm_max_id = 0 as u64; //We need the actual max id to delete last message

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
                       process_tweet(&tweet, &consumer, &access, &config);
                    }
                    //must be a better way to do this?
                    if (tweet.id > dm_max_id) && (tweet.user.screen_name == config.twitter.screen_name.to_string()) {
                        dm_max_id = tweet.id;
                    }
                }
            }
        }

        counter::set(new_max_id, "status.id").unwrap();
        process_dms(&consumer, &access, &config, &dm_max_id);
        thread::sleep(Duration::from_secs(60)); //Run every 60 seconds...
    }
}

fn process_dms(consumer: &Token, access: &Token, config: &TwatterConfig, last_tweet_id: &u64) {
    let dms = get_direct_messages(&consumer, &access);
    if ! dms.is_none() {
        let dms = dms.unwrap();
        if ! dms.is_empty() {
            for dm in dms {
                if ! config.aliases[&dm.sender_screen_name].as_str().unwrap().is_empty() {
                    process_dm_command(&consumer, &access, &dm, &last_tweet_id);
                }
            }
        }
    }
}

fn process_dm_command(consumer: &Token, access: &Token, dm: &DirectMessage, last_tweet_id: &u64) {
    let max_id = counter::get("dm.id") as u64;
    if dm.id > max_id {
        match dm.text.to_uppercase().as_str() {
            "DELETE" => {
                delete_tweet(&consumer, &access, &last_tweet_id);
            },
            _ => {
                println!("UNKNOWN DIRECT MESSAGE COMMAND");
                match twitter_api::direct_message(&consumer, &access, "UNKNOWN DIRECT MESSAGE COMMAND", &dm.sender_screen_name) {
                    Err(e) => println!("Failed to send DM, are they following us? {}", e),
                    Ok(v) => (v)
                }
            }
        };
        counter::set(dm.id, "dm.id").unwrap();
    }
}

fn delete_tweet(consumer: &Token, access: &Token, last_tweet_id: &u64) {
    println!("\tDELETING {}", last_tweet_id.to_string());
    match twitter_api::destroy_status(&consumer, &access, &last_tweet_id) {
        Err(e) => println!("Failed to Delete Tweet {}", e),
        Ok(v) => (v)
    };
}

fn process_tweet(tweet: &Tweet, consumer:&Token, access:&Token, config: &TwatterConfig) {
    if tweet.user.screen_name != config.twitter.screen_name.to_string() {
        retweet(&tweet, &consumer, &access, &config);
    }
}

fn retweet(tweet: &Tweet, consumer:&Token, access:&Token, config: &TwatterConfig) {
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

fn get_direct_messages(consumer:&Token, access:&Token)->Option<Vec<DirectMessage>> {
    match twitter_api::get_direct_messages(&consumer, &access) {
        Err(e) => {
            println!("{:?}", e);
            None
        },
        Ok(messages) => Some(messages),
    }

}




