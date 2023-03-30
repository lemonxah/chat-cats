mod love;
mod slap;
mod stats;
mod hug;
mod remind;
mod time;
pub use love::*;
pub use slap::*;
pub use stats::*;
pub use hug::*;
pub use remind::*;
pub use time::*;

use thiserror::Error;
use std::{result::Result, sync::Arc};
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
    #[error(transparent)]
    TimeError(#[from] chrono::ParseError),
}

pub trait HelpCommands {
    fn help() -> Vec<&'static str>;
}

#[async_trait]
pub trait Responder {
    type Config;
    fn new(config: Self::Config) -> Self where Self: Sized;
    async fn respond(&self, message: &Message, discord: Arc<Discord>, db: Database) -> Result<Message, CommandError>;
}

#[async_trait]
pub trait ChatCommand {
    fn help(&self) -> Vec<&'static str>;
    fn matches(&self, message: &str) -> bool;
    async fn handle(&self, message: &Message, discord: Arc<Discord>, db: Database) -> Result<Message, CommandError>;
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

    pub async fn handle(&self, message: &Message, discord: Arc<Discord>, db: Database) -> Result<(), Box<dyn std::error::Error>> {
        let re_string = format!("(?i)({})", self.names.join("|"));
        let is_command = Regex::new(&re_string).unwrap();
        if is_command.is_match(&message.content) {
            let stripped = is_command.replacen(&message.content, 1, "").to_string();
            let help = Regex::new(r"(?i)^(help|commands|what can you do)$").unwrap();
            if help.is_match(&stripped.trim()) {
                let pc = discord.create_private_channel(message.author.id)?;
                let mut help = vec!["```", "I can do the following commands:", "   help - Responds with this message"];
                for command in &self.commands {
                    help.append(&mut command.help());
                }
                help.push("```");
                let _ = discord.send_message(pc.id, &help.join("\n"), "", false);
                return Ok(());
            }
            for command in &self.commands {
                if command.matches(stripped.trim()) {
                    command.handle(message, discord.clone(), db.clone()).await?;
                }
            }
        }
        Ok(())
    }
}
