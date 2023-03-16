use async_trait::async_trait;
use discord::{model::Message, Discord};
use macros::ChatCommand;
use regex::Regex;
use serde::Deserialize;
use std::result::Result;
use rand::seq::SliceRandom;
use mongodb::Database;
use super::{ChatCommand, CommandError, Responder, HelpCommands};


#[derive(Deserialize, Debug, Clone)]
pub struct HugConfig {
    responses: Vec<String>,
}

#[derive(ChatCommand)]
pub struct HugCommand {
    matches: Vec<&'static str>,
    config: HugConfig,
}

impl HelpCommands for HugCommand {
    fn help() -> Vec<&'static str> {
        vec![
            "   hug <target> - Responds with a random hug message",
            "       Example: cc hug @user",
        ]
    }
}

#[async_trait]
impl Responder for HugCommand {
    type Config = HugConfig;
    fn new(config: Self::Config) -> HugCommand {
        HugCommand { 
            matches: vec![
                "hug",
            ],
            config,
        }
    }
    async fn respond(&self, message: &Message, discord: &Discord, _db: Database) -> Result<Message, CommandError> {
        let oponent_regex = Regex::new("<@(\\d+?)>").unwrap();
        let oponent_test = message.content.to_string();
        if let Some(oponent_match) = oponent_regex.find(&oponent_test) {
            let response = {
                let rng = &mut rand::thread_rng();
                format!("<@{}> gives {} a {}", message.author.id, oponent_test[oponent_match.start()..oponent_match.end()].to_string(), self.config.responses.choose(rng).unwrap())
            };
            discord.send_message(message.channel_id, &response, "", false).map_err(|e| e.into())
        } else {
            discord.send_message(message.channel_id, "No target for the hug was specified", "", false).map_err(|e| e.into())
        }
    }
}
