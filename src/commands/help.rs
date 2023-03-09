use async_trait::async_trait;
use macros::ChatCommand;
use serde::Deserialize;
use std::result::Result;
use discord::{model::Message, Discord};
use mongodb::Database;
use super::{ChatCommand, CommandError, Responder, HelpCommands};

#[derive(Deserialize, Debug, Clone)]
pub struct HelpConfig {
    pub response: Vec<String>,
}

#[derive(ChatCommand)]
pub struct HelpCommand {
    matches: Vec<&'static str>,
    config: HelpConfig,
}

impl HelpCommands for HelpCommand {
    fn help() -> Vec<&'static str> {

    }
}

#[async_trait]
impl Responder for HelpCommand {
    type Config = HelpConfig;
    fn new(config: Self::Config) -> HelpCommand {
        HelpCommand { 
            matches: vec![
                "commands",
                "help",
                "what can you do",
            ],
            config,
        }
    }
    async fn respond(&self, message: &Message, discord: &Discord, _db: Database) -> Result<Message, CommandError> {
        let pc = discord.create_dm(message.author.id).unwrap();
        discord.send_message(pc.id, &self.config.response.join("\n"), "", false).map_err(|e| e.into())
    }
}
