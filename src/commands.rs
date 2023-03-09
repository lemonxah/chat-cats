mod love;
mod slap;
mod help;
mod stats;
pub use love::*;
pub use slap::*;
pub use help::*;
pub use stats::*;

use thiserror::Error;
use std::result::Result;
use async_trait::async_trait;
use discord::{model::Message, Discord};
use mongodb::Database;
use regex::Regex;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error(transparent)]
    DatabaseError(#[from] mongodb::error::Error),
    #[error(transparent)]
    DiscordError(#[from] discord::Error),
    #[error(transparent)]
    RegexError(#[from] regex::Error),
    #[error(transparent)]
    CastError(#[from] std::num::ParseIntError),
}

#[async_trait]
pub trait Responder {
    type Config;
    fn new(config: Self::Config) -> Self where Self: Sized;
    async fn respond(&self, message: &Message, discord: &Discord, db: Database) -> Result<Message, CommandError>;
}

#[async_trait]
pub trait ChatCommand {
    fn matches(&self, message: &str) -> bool;
    async fn handle(&self, message: &Message, discord: &Discord, db: Database) -> Result<Message, CommandError>;
}

pub struct Commands {
    names: Vec<&'static str>,
    commands: Vec<Box<dyn ChatCommand>>
}

impl Commands {
    pub fn new() -> Commands {
        Commands { names: vec!["cc", "chat-cats"], commands: vec![] }
    }

    pub fn add_command(&mut self, command: Box<dyn ChatCommand>) -> &mut Self {
        self.commands.push(command);
        self
    }

    pub async fn handle(&self, message: &Message, discord: &Discord, db: Database) -> Result<(), Box<dyn std::error::Error>> {
        let re_string = format!("(?i)({})", self.names.join("|"));
        let is_command = Regex::new(&re_string).unwrap();
        if is_command.is_match(&message.content) {
            let stripped = is_command.replacen(&message.content, 1, "").to_string();
            for command in &self.commands {
                if command.matches(stripped.trim()) {
                    command.handle(message, discord, db.clone()).await?;
                }
            }
        }
        Ok(())
    }
}
