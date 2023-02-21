mod love;
mod slap;
pub use love::*;
use rand::rngs::ThreadRng;
pub use slap::*;

use discord::{model::Message, Discord, Result};
use regex::Regex;

use crate::Config;

pub trait ChatCommand {
    fn matches(&self, message: &str) -> bool;
    fn handle(&self, message: &Message, discord: &Discord, config: &Config, rng: &mut ThreadRng) -> Result<Message>;
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

    pub fn handle(&self, message: &Message, discord: &Discord, config: &Config, rng: &mut ThreadRng) {
        let re_string = format!("(?i)({})", self.names.join("|"));
        let is_command = Regex::new(&re_string).unwrap();
        if is_command.is_match(&message.content) {
            let stripped = is_command.replacen(&message.content, 1, "").to_string();
            for command in &self.commands {
                if command.matches(stripped.trim()) {
                    let result = command.handle(message, discord, config, rng);
                    if let Err(error) = result {
                        println!("error running command with input message: {}, error: {}", message.content, error);
                    }
                }
            }
        }
    }
}
