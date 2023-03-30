mod profanity;
mod commands;
mod storage;
mod cache;

#[macro_use] extern crate log;

extern crate discord;
use discord::model::Event;
use discord::Discord;
use mongodb::{Client, options::ClientOptions};
use serde_derive::Deserialize;
use std::env;
use std::sync::Arc;
use dotenv::dotenv;

use crate::profanity::profanity_filter;
use crate::commands::*;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub profanity: profanity::ProfanityConfig,
    pub slap: commands::SlapConfig,
    pub love: commands::LoveConfig,
    pub hug: commands::HugConfig,
    pub remind: commands::RemindConfig,
    pub time: commands::TimeConfig,
    pub db: DBConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DBConfig {
    pub connection_string: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv().ok();
    let config_str = std::fs::read_to_string("config.toml").expect("config.toml file missing");
    let config: Config = toml::from_str(&config_str).expect("config.toml format broken");
    info!("Config loaded:\n{:?}\n", config);
    let token = env::var("DISCORD_TOKEN").expect("token expected");
    let discord = Discord::from_bot_token(&token).expect("login failed");
    info!("Login successful");
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
        .add_command(Box::new(HugCommand::new(config.hug)))
        .add_command(Box::new(RemindCommand::new(config.remind)))
        .add_command(Box::new(TimeCommand::new(config.time)))
        .add_command(Box::new(StatsCommand::new(())));
    let discord = Arc::new(discord);
    info!("Ready.");
    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                info!("{} says: {}", message.author.name, message.content);
                profanity_filter(&message, &config.profanity, discord.clone());
                if let Err(e) = commands.handle(&message, discord.clone(), db.clone()).await {
                    error!("Error handling command: {} ::: {}", message.content, e);
                };
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                error!("Gateway closed on us with code {:?}: {}", code, body);
                break;
            }
            Err(err) => error!("Receive error: {:?}", err),
        }
    }
    Ok(())
}
