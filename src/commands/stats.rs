use async_trait::async_trait;
use macros::ChatCommand;
use std::result::Result;
use discord::{model::Message, Discord};
use mongodb::Database;
use super::{ChatCommand, CommandError, Responder};

#[derive(ChatCommand)]
pub struct StatsCommand {
    matches: Vec<&'static str>,
}

#[async_trait]
impl Responder for StatsCommand {
    type Config = ();
    fn new(_config: ()) -> StatsCommand {
        StatsCommand { 
            matches: vec![
                "stats",
                "slaps",
            ], 
        }
    }
    async fn respond(&self, message: &Message, _cardiscord: &Discord, _db: Database) -> Result<Message, CommandError> {
        Ok(message.clone())
    }
}
