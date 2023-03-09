mod profanity;
mod commands;
mod storage;
mod cache;

extern crate discord;
use discord::model::Event;
use discord::Discord;
use mongodb::{Client, options::ClientOptions};
use serde_derive::Deserialize;
use std::env;
use dotenv::dotenv;

use crate::profanity::profanity_filter;
use crate::commands::*;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub profanity: profanity::ProfanityConfig,
	pub slap: commands::SlapConfig,
	pub love: commands::LoveConfig,
	pub db: DBConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DBConfig {
	pub connection_string: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let config_str = std::fs::read_to_string("config.toml").expect("config.toml file missing");
    let config: Config = toml::from_str(&config_str).expect("config.toml format broken");
	println!("Config loaded:\n{:?}\n", config);
    let token = env::var("DISCORD_TOKEN").expect("token expected");
    let discord = Discord::from_bot_token(&token).expect("login failed");
	println!("Login successful");
	let mut client_options = ClientOptions::parse(config.db.connection_string).await?;
	client_options.app_name = Some("Chat Cats".to_string());
	let client = Client::with_options(client_options)?;
	let db = client.database("chat-cats");
	// Establish and use a websocket connection
	let (mut connection, _) = discord.connect().expect("connect failed");
    let mut commands = Commands::new();
    commands
        .add_command(Box::new(LoveCommand::new(config.love)))
        .add_command(Box::new(SlapCommand::new(config.slap)))
		.add_command(Box::new(StatsCommand::new(())));

	println!("Ready.");
	loop {
		match connection.recv_event() {
			Ok(Event::MessageCreate(message)) => {
				println!("{} says: {}", message.author.name, message.content);
                profanity_filter(&message, &config.profanity, &discord);
                commands.handle(&message, &discord, db.clone()).await?;
			}
			Ok(_) => {}
			Err(discord::Error::Closed(code, body)) => {
				println!("Gateway closed on us with code {:?}: {}", code, body);
				break;
			}
			Err(err) => println!("Receive error: {:?}", err),
		}
	}
	Ok(())
}
