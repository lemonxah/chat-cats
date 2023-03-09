use async_trait::async_trait;
use discord::{model::Message, Discord};
use macros::ChatCommand;
use serde::Deserialize;
use std::result::Result;
use rand::seq::SliceRandom;
use mongodb::Database;
use super::{ChatCommand, CommandError, Responder, HelpCommands};


#[derive(Deserialize, Debug, Clone)]
pub struct LoveConfig {
    responses: Vec<String>,
}

#[derive(ChatCommand)]
pub struct LoveCommand {
    matches: Vec<&'static str>,
    config: LoveConfig,
}

impl HelpCommands for LoveCommand {
    fn help() -> Vec<&'static str> {
        vec![
            "   love you - Responds with a random love message",
            "       Example: cc love you",
            "       Example: cc ❤️",
        ]
    }
}

#[async_trait]
impl Responder for LoveCommand {
    type Config = LoveConfig;
    fn new(config: Self::Config) -> LoveCommand {
        LoveCommand { 
            matches: vec![
                "love you",
                "❤️",
            ],
            config,
        }
    }
    async fn respond(&self, message: &Message, discord: &Discord, _db: Database) -> Result<Message, CommandError> {
        let response = {
            let rng = &mut rand::thread_rng();
            format!("<@{}>, {}", message.author.id, self.config.responses.choose(rng).unwrap())
        };
        discord.send_message(message.channel_id, &response, "", false).map_err(|e| e.into())
    }
}
