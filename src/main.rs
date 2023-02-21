mod profanity;
mod commands;

extern crate discord;
use discord::model::Event;
use discord::Discord;

use serde_derive::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;

use crate::profanity::profanity_filter;
use crate::commands::{Commands, LoveCommand, SlapCommand};

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
    let discord = Discord::from_bot_token(&token).expect("login failed");

	// Establish and use a websocket connection
	let (mut connection, _) = discord.connect().expect("connect failed");
    let mut commands = Commands::new();
    commands
        .add_command(Box::new(LoveCommand::new()))
        .add_command(Box::new(SlapCommand::new()));

	println!("Ready.");
	loop {
		match connection.recv_event() {
			Ok(Event::MessageCreate(message)) => {
				println!("{} says: {}", message.author.name, message.content);
                profanity_filter(&message, &config, &discord);
                commands.handle(&message, &discord, &config);
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
