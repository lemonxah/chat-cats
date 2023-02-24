use async_trait::async_trait;
use macros::ChatCommand;
use std::result::Result;
use discord::{model::Message, Discord};
use crate::Config;
use mongodb::Database;
use super::{ChatCommand, CommandError};

#[derive(ChatCommand)]
pub struct HelpCommand {
    matches: Vec<&'static str>,
    response: Vec<&'static str>,
}

impl HelpCommand {
    pub fn new() -> HelpCommand {
        HelpCommand { 
            matches: vec![
                "!commands",
                "commands",
                "help",
                "what can you do",
                "what can you do?",
            ],
            response: vec![
                "```",
                "I can do a lot of things! Here's a list of commands I can respond to:",
                "   love you - Responds with a random love message",
                "       Example: cc love you",
                "   slap @target - Responds with a random slap message aimed at the @target user",
                "       Example: cc slap @ChatCats",
                "       Example: cc please slap @ChatCats",
                "   slaps - Shows the User's effective slaps on @targets",
                "   stats - Responds with some stats about the bot and the chatters",
                "   help - Responds with this message",
                "```",
            ],
        }
    }
    pub async fn respond(&self, message: &Message, discord: &Discord, _db: Database) -> Result<Message, CommandError> {
        let pc = discord.create_dm(message.author.id).unwrap();
        discord.send_message(pc.id, &self.response.join("\n"), "", false).map_err(|e| e.into())
    }
}
