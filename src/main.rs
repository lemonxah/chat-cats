extern crate discord;
use rand::{thread_rng, Rng};
use discord::model::{Event, Message};
use discord::Discord;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub bad_words: Vec<String>,
    pub replacements: Vec<String>,
}

fn main() {
    dotenv().ok();
    let config_str = std::fs::read_to_string("config.toml").expect("config.toml file missing");
    let config: Config = toml::from_str(&config_str).expect("config.toml format broken");
    let token = env::var("DISCORD_TOKEN").expect("token expected");
    let discord = Discord::from_bot_token(&token)
		.expect("login failed");

	// Establish and use a websocket connection
	let (mut connection, _) = discord.connect().expect("connect failed");
	println!("Ready.");
	loop {
		match connection.recv_event() {
			Ok(Event::MessageCreate(message)) => {
				println!("{} says: {}", message.author.name, message.content);
                bad_words_fix(&message, &config, &discord);
                commands(&message, &discord, &config);
			}
			Ok(_) => {}
			Err(discord::Error::Closed(code, body)) => {
				println!("Gateway closed on us with code {:?}: {}", code, body);
				break;
			}
			Err(err) => println!("Receive error: {:?}", err),
		}
	}
}


pub fn commands(message: &Message, discord: &Discord, config: &Config) {
    let love = || {
        let _ = discord.send_message(message.channel_id, &format!("THANK YOU <@{}> i love you too!! ❤️❤️❤️", message.author.id), "", false);
    };
    
    match message.content.as_str() {
        "hey chat-cats i love you" => love(),
        "hey chat cats i love you" => love(),
        "hey chatcats i love you" => love(),
        "hey cc i love you" => love(),
        "love you cc" => love(),
        "love you chat cats" => love(),
        "love you chatty" => love(),
        "❤️ cc" => love(),
        "❤️ chat cats" => love(),
        "❤️ chat-cats" => love(),
        "❤️ chatty" => love(),
        _ => {},
    }
}

pub fn bad_words_fix(message: &Message, config: &Config, discord: &Discord) {
    let mut rng = thread_rng();
    let n = rng.gen_range(0..config.replacements.len()-1);
    let re_string = format!("(?i)({})", config.bad_words.join("|"));
    let re = Regex::new(&re_string).unwrap();
    if re.is_match(&message.content) {
        let new_text = re.replace_all(&message.content, &config.replacements[n]);
        let text = format!("I am sure <@{}> meant to say: {}", message.author.id, new_text.to_string());
        let _ = discord.delete_message(message.channel_id, message.id);
        let _ = discord.send_message(message.channel_id, &text, "", false);
    }
}