use async_trait::async_trait;
use discord::{model::Message, Discord};
use macros::ChatCommand;
use serde::Deserialize;
use std::{result::Result, sync::Arc};
use mongodb::Database;
use regex::Regex;
use super::{ChatCommand, CommandError, Responder, HelpCommands};


#[derive(Deserialize, Debug, Clone)]
pub struct DefinitionsConfig {
    response: Vec<String>,
}

#[derive(ChatCommand)]
pub struct DefinitionsCommand {
    matches: Vec<&'static str>,
    config: DefinitionsConfig,
}

impl HelpCommands for DefinitionsCommand {
    fn help() -> Vec<&'static str> {
        vec![
            "   Definitions - Responds with a Definitions message",
            "       Example: cc definitions",
            "       Alias: cc explain, cc abv"
        ]
    }
}

#[async_trait]
impl Responder for DefinitionsCommand {
    type Config = DefinitionsConfig;
    fn new(config: Self::Config) -> DefinitionsCommand {
        DefinitionsCommand { 
            matches: vec![
                "definitions",
                "abv",
                "explain",
            ],
            config,
        }
    }
    async fn respond(&self, message: &Message, discord: Arc<Discord>, _db: Database) -> Result<Message, CommandError> {
        let response = {
            format!("{}", self.config.response.join("\n"))
        };
        discord.send_message(message.channel_id, &response, "", false).map_err(|e| e.into())
    }
}
