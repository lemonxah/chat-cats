use async_trait::async_trait;
use discord::{model::{Message, UserId}, Discord};
use macros::ChatCommand;
use rand::{seq::SliceRandom, rngs::ThreadRng};
use serde::{Deserialize, Serialize};
use std::result::Result;
use regex::Regex;
use strum::{EnumIter, IntoEnumIterator};
use crate::storage;
use mongodb::Database;
use super::{ChatCommand, CommandError, Responder, HelpCommands};

#[derive(Deserialize, Debug, Clone)]
pub struct SlapConfig {
    pub actions: Vec<String>,
    pub objects: Vec<String>,
    pub body_parts: Vec<String>,
    pub results: Vec<String>,
}

#[derive(ChatCommand)]
pub struct SlapCommand {
    matches: Vec<&'static str>,
    config: SlapConfig,
}

type Oponent = String;

#[derive(Deserialize, Serialize, Debug, EnumIter, PartialEq, Eq, Hash)]
enum SubCommand {
    #[serde(rename = "slap help")]
    Help,
    #[serde(rename = "slap stats")]
    Stats,
    #[serde(rename = "slap leaderboard")]
    Leaderboard,
    #[serde(rename = "slap top")]
    Top,
    #[serde(rename = "slap random")]
    Random,
    #[serde(rename = "slap")]
    Slap(Oponent),
}

impl Into<&'static str> for SubCommand {
    fn into(self) -> &'static str {
        match self {
            SubCommand::Help => "slap help",
            SubCommand::Stats => "slap stats",
            SubCommand::Leaderboard => "slap leaderboard",
            SubCommand::Top => "slap top",
            SubCommand::Random => "slap random",
            SubCommand::Slap(_) => "slap",
        }
    }
}

impl HelpCommands for SlapCommand {
    fn help() -> Vec<&'static str> {
        vec![
            "   slap help - Responds with a list of slap commands",
            "       Example: cc slap help",
            "   slap stats - Responds with stats about your slaps",
            "   slap leaderboard - Responds with the slap leaderboard",
            "       Example: cc slap leaderboard",
            "       Example: cc slap top",
            "   slap random - Responds with a random slap message aimed at a random online user",
            "       Example: cc slap random",
            "   slap @target - Responds with a random slap message aimed at the @target user",
            "       Example: cc slap @ChatCats",        
        ]
    }
}

impl From<&Message> for SubCommand {
    fn from(message: &Message) -> Self {
        let subcommand = message.content.replace("cc slap", "");
        let mut split = subcommand.trim().split_whitespace();
        let command = split.next().unwrap();
        println!("sub command: {}", command);
        match command {
            "help" => SubCommand::Help,
            "stats" => SubCommand::Stats,
            "leaderboard" => SubCommand::Leaderboard,
            "top" => SubCommand::Top,
            "random" => SubCommand::Random,
            oponent_test => {
                println!("its here!, {}", oponent_test);
                let oponent_regex = Regex::new("<@(\\d+?)>").unwrap();
                if let Some(oponent_match) = oponent_regex.find(oponent_test) {
                    SubCommand::Slap(oponent_test[oponent_match.start()..oponent_match.end()].to_string())
                } else {
                    SubCommand::Help
                }
            },
        }
    }
}

impl SlapCommand {
    pub async fn slap(&self, oponent: Oponent, message: &Message, discord: &Discord, db: Database) -> Result<Message, CommandError> {
        let response = {
            let mut rng: ThreadRng = rand::thread_rng();
            let action = self.config.actions.choose(&mut rng).unwrap();
            let object = self.config.objects.choose(&mut rng).unwrap();
            let body_part = self.config.body_parts.choose(&mut rng).unwrap();
            let result = self.config.results.choose(&mut rng).unwrap();
            let result = result.replace("<@>", &oponent);
            format!("<@{}> {} {} with {} {} and {}", message.author.id, action, oponent, object, body_part, result)
        };
        if !response.contains("!!!") {
            // slap counts as valid and is recorded
            let _slaps = storage::slaps::slap(message.author.id.0 as i64, oponent[2..oponent.len()-1].parse::<i64>().unwrap(), db).await?;
        }
        discord.send_message(message.channel_id, &response.replace("!!!", ""), "", false).map_err(|e| e.into())
    }

    pub async fn help(&self, message: &Message, discord: &Discord) -> Result<Message, CommandError> {
        let response = format!("Usage: slap <oponent>");
        discord.send_message(message.channel_id, &response, "", false).map_err(|e| e.into())
    }

    pub async fn stats(&self, message: &Message, discord: &Discord, db: Database) -> Result<Message, CommandError> {
        // let stats = storage::slaps::stats(message.author.id.0 as i64, db).await?;
        let given = storage::slaps::given(message.author.id.0 as i64, db.clone()).await?;
        let received = storage::slaps::received(message.author.id.0 as i64, db).await?;
        let response = format!("<@{}> has slapped {} people, and have been slapped {} times", message.author.id, given, received);
        discord.send_message(message.channel_id, &response, "", false).map_err(|e| e.into())
    }

    fn get_name(id: i64, discord: &Discord) -> Result<String, CommandError> {
        let id = UserId(id as u64);
        let user = discord.get_user(id)?;
        Ok(user.name)
    }

    pub async fn leaderboard(&self, message: &Message, discord: &Discord, db: Database) -> Result<Message, CommandError> {
        let slappers = storage::slaps::top3_slappers(db.clone()).await?;
        let slappees = storage::slaps::top3_slappees(db).await?;
        let response = format!("```Top 3 Slappers:\n\n{} \n\nTop 3 Slapped:\n\n{}\n\n```",
            slappers.iter().map(|s| format!("  {} with {} slaps", Self::get_name(s.author, discord).unwrap_or("unknown".to_owned()), s.count)).collect::<Vec<String>>().join("\n"),
            slappees.iter().map(|s| format!("  {} with {} slaps", Self::get_name(s.author, discord).unwrap_or("unknown".to_owned()), s.count)).collect::<Vec<String>>().join("\n")
        );
        discord.send_message(message.channel_id, &response, "", false).map_err(|e| e.into())
    }

}

#[async_trait]
impl Responder for SlapCommand {
    type Config = SlapConfig;
    fn new(config: SlapConfig) -> SlapCommand {
        SlapCommand { 
            matches: SubCommand::iter().map(|s| s.into()).collect(),
            config,
        }
    }

    async fn respond(&self, message: &Message, discord: &Discord, db: Database) -> Result<Message, CommandError> {
        match SubCommand::from(message) {
            SubCommand::Slap(oponent) => self.slap(oponent, message, discord, db).await,
            SubCommand::Help => self.help(message, discord).await,
            SubCommand::Stats => self.stats(message, discord, db).await,
            SubCommand::Leaderboard =>self.leaderboard(message, discord, db).await,
            SubCommand::Top => self.leaderboard(message, discord, db).await,
            SubCommand::Random => todo!(),
        }
    }
}