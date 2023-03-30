use async_trait::async_trait;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use discord::{model::Message, Discord};
use macros::ChatCommand;
use serde::Deserialize;
use std::{result::Result, sync::Arc};
use regex::Regex;

use mongodb::Database;
use super::{ChatCommand, CommandError, Responder, HelpCommands};

#[derive(Deserialize, Debug, Clone)]
pub struct TimeConfig {

}

#[derive(ChatCommand)]
pub struct TimeCommand {
    matches: Vec<&'static str>,
    _config: TimeConfig,
}


impl HelpCommands for TimeCommand {
    fn help() -> Vec<&'static str> {
        vec![
            "   time <timezone> - Responds with the time at the specified timezone",
            "       Example: cc time sast",
            "       Example: cc time est",
        ]
    }
}

impl TimeCommand {
    pub async fn _help(&self, message: &Message, discord: Arc<Discord>) -> Result<Message, CommandError> {
        let response = format!("Time Commands: \n{}", self.help().join("\n"));
        discord.send_message(message.channel_id, &response, "", false).map_err(|e| e.into())
    }

    pub async fn time_at(&self, message: &Message, discord: Arc<Discord>) -> Result<Message, CommandError> {
        let tz = Regex::new(r"cc time (?P<timezone>\w+)").unwrap();
        let timezone = if tz.is_match(&message.content) { 
            let caps = tz.captures(&message.content).unwrap();
            caps.name("timezone").unwrap().as_str() 
        } else { 
            "utc" 
        };
        let timezone = match timezone.to_lowercase().as_str() {
            "utc" => "UTC",
            "est" => "America/New_York",
            "cst" => "America/Chicago",
            "mst" => "America/Denver",
            "pst" => "America/Los_Angeles",
            "sast" => "Africa/Johannesburg",
            "uk" => "Europe/London",
            _ => "UTC",
        };
        let tz: Tz = timezone.parse().unwrap();
        let now: DateTime<Tz> = Utc::now().with_timezone(&tz);
        let response = format!("<@{}>, the time is {}", message.author.id, now.format("%H:%M:%S %Z"));
        discord.send_message(message.channel_id, &response, "", false).map_err(|e| e.into())
    }
}

#[async_trait]
impl Responder for TimeCommand {
    type Config = TimeConfig;
    fn new(config: TimeConfig) -> TimeCommand {
        TimeCommand { 
            matches: vec![
                "time",
            ],
            _config: config,
        }
    }

    async fn respond(&self, message: &Message, discord: Arc<Discord>, _db: Database) -> Result<Message, CommandError> {
        self.time_at(message, discord).await
    }
}