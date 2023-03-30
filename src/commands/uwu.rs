use async_trait::async_trait;
use discord::{model::Message, Discord};
use macros::ChatCommand;
use serde::Deserialize;
use std::{result::Result, sync::Arc};
use rand::seq::SliceRandom;
use mongodb::Database;
use super::{ChatCommand, CommandError, Responder, HelpCommands};


#[derive(Deserialize, Debug, Clone)]
pub struct UwuConfig {
    responses: Vec<String>,
}

#[derive(ChatCommand)]
pub struct UwuCommand {
    matches: Vec<&'static str>,
    config: UwuConfig,
}

impl HelpCommands for UwuCommand {
    fn help() -> Vec<&'static str> {
        vec![
            "   uwu - Responds with a random uwu message",
            "       Example: cc uwu",
        ]
    }
}

#[async_trait]
impl Responder for UwuCommand {
    type Config = UwuConfig;
    fn new(config: Self::Config) -> UwuCommand {
        UwuCommand { 
            matches: vec![
                "uwu",
            ],
            config,
        }
    }
    async fn respond(&self, message: &Message, discord: Arc<Discord>, _db: Database) -> Result<Message, CommandError> {
        let response = {
            let rng = &mut rand::thread_rng();
            format!("<@{}>, {}", message.author.id, self.config.responses.choose(rng).unwrap())
        };
        discord.send_message(message.channel_id, &response, "", false).map_err(|e| e.into())
    }
}
