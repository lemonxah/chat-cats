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
            "   time <timezone> <time> in <timezone> - Responds with the time at the specified timezone",
            "       Example: cc time sast 11:40 in est",
            "       Example: cc time est 11:40 in uk",
        ]
    }
}

impl TimeCommand {
    pub async fn _help(&self, message: &Message, discord: Arc<Discord>) -> Result<Message, CommandError> {
        let response = format!("Time Commands: \n{}", self.help().join("\n"));
        discord.send_message(message.channel_id, &response, "", false).map_err(|e| e.into())
    }

    fn get_timezone(timezone: &str) -> &'static str {
        match timezone.to_lowercase().as_str() {
            "utc" => "UTC",
            "est" => "America/New_York",
            "cst" => "America/Chicago",
            "mst" => "America/Denver",
            "pst" => "America/Los_Angeles",
            "sast" => "Africa/Johannesburg",
            "uk" => "Europe/London",
            _ => "UTC",
        }        
    }

    pub async fn convert_time(&self, message: &Message, discord: Arc<Discord>) -> Result<Message, CommandError> {
        let tz = Regex::new(r"cc time (?P<timezone>\w+) (?P<time>\d{1,2}:\d{2}) in (?P<timezone2>\w+)").unwrap();
        let timezone = if tz.is_match(&message.content) { 
            let caps = tz.captures(&message.content).unwrap();
            caps.name("timezone").unwrap().as_str() 
        } else { 
            "utc" 
        };
        let timezone2 = if tz.is_match(&message.content) { 
            let caps = tz.captures(&message.content).unwrap();
            caps.name("timezone2").unwrap().as_str() 
        } else { 
            "utc" 
        };
        let time = if tz.is_match(&message.content) { 
            let caps = tz.captures(&message.content).unwrap();
            caps.name("time").unwrap().as_str() 
        } else { 
            "00:00" 
        };
        let timezone = TimeCommand::get_timezone(timezone);
        let timezone2 = TimeCommand::get_timezone(timezone2);
        let tz: Tz = timezone.parse().unwrap();
        let tz2: Tz = timezone2.parse().unwrap();
        let now: DateTime<Tz> = Utc::now().with_timezone(&tz);
        let timestr = &format!("{} {}:00 {}", now.format("%Y-%m-%d"), time, now.format("%z"));
        let time1: DateTime<Tz> = DateTime::parse_from_str(timestr, "%Y-%m-%d %H:%M:%S %z")?.with_timezone(&tz);
        let time2: DateTime<Tz> = DateTime::parse_from_str(timestr, "%Y-%m-%d %H:%M:%S %z")?.with_timezone(&tz2);
        let response = format!("<@{}>, {} is {}", message.author.id, time1.format("%H:%M:%S %Z"), time2.format("%H:%M:%S %Z"));
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
        let timezone = TimeCommand::get_timezone(timezone);
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
        let time_at = Regex::new(r"cc time (?P<timezone>\w+)$").unwrap();
        let time_convert = Regex::new(r"cc time (?P<timezone>\w+) (?P<time>\d{1,2}:\d{2}) in (?P<timezone2>\w+)").unwrap();
        if time_at.is_match(&message.content) {
            return self.time_at(message, discord).await;
        } else if time_convert.is_match(&message.content) {
            return self.convert_time(message, discord).await;
        } else {
            self._help(message, discord).await
        }
    }
}