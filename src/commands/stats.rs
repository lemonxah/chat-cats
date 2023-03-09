use async_trait::async_trait;
use macros::ChatCommand;
use std::result::Result;
use discord::{model::Message, Discord};
use mongodb::Database;
use super::{ChatCommand, CommandError, Responder, HelpCommands};

#[derive(ChatCommand)]
pub struct StatsCommand {
    matches: Vec<&'static str>,
}

impl HelpCommands for StatsCommand {
    fn help() -> Vec<&'static str> {
        vec![
            "   stats - Responds with a stats message",
            "       Example: cc stats",
        ]
    }
}

#[async_trait]
impl Responder for StatsCommand {
    type Config = ();
    fn new(_config: ()) -> StatsCommand {
        StatsCommand { 
            matches: vec![
                "stats",
            ], 
        }
    }
    async fn respond(&self, message: &Message, _cardiscord: &Discord, _db: Database) -> Result<Message, CommandError> {
        Ok(message.clone())
    }
}
