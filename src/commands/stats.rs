use async_trait::async_trait;
use macros::ChatCommand;
use std::result::Result;
use discord::{model::Message, Discord};
use crate::Config;
use mongodb::Database;
use super::{ChatCommand, CommandError};

#[derive(ChatCommand)]
pub struct StatsCommand {
    matches: Vec<&'static str>,
}

impl StatsCommand {
    pub fn new() -> StatsCommand {
        StatsCommand { 
            matches: vec![
                "stats",
                "slaps",
            ], 
        }
    }
    pub async fn respond(&self, message: &Message, _cardiscord: &Discord, _db: Database) -> Result<Message, CommandError> {
        // let pc = discord.create_dm(message.author.id).unwrap();
        // discord.send_message(pc.id, &self.response.join("\n"), "", false).map_err(|e| e.into())
        Ok(message.clone())
    }
}
