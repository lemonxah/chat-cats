use async_trait::async_trait;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use discord::{model::Message, Discord};
use macros::ChatCommand;
use serde::{Deserialize, Serialize};
use std::{result::Result, sync::Arc};
use regex::Regex;
use tokio::time::{Duration, sleep};

use mongodb::Database;
use super::{ChatCommand, CommandError, Responder, HelpCommands};


#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct RemindIn {
    pub seconds: u64,
    pub what: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct RemindAt {
    pub time: String,
    pub timezone: String,
    pub what: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash)]
enum SubCommand {
    #[serde(rename = "remind me in")]
    RemindIn(RemindIn),
    #[serde(rename = "remind me at")]
    RemindAt(RemindAt),
    #[serde(rename = "remind help")]
    Help,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RemindConfig {

}

#[derive(ChatCommand)]
pub struct RemindCommand {
    matches: Vec<&'static str>,
    _config: RemindConfig,
}


impl HelpCommands for RemindCommand {
    fn help() -> Vec<&'static str> {
        vec![
            "   remind me in x [seconds|minutes|hours] to <what you should be reminded of> - Responds with a reminder",
            "       Example: cc remind me in 10 minutes to take a break",
            "       Example: cc remind me in 2 hours to pick up the kids",
            "   remind me at <time> <timezone> to <what you should be reminded of> - Responds with a reminder",
            "       Example: cc remind me at 11:40 GMT to start lunch",
        ]
    }
}

struct RemTime {
    hours: u64,
    minutes: u64,
    seconds: u64,
}

impl RemTime {
    fn from_seconds(seconds: u64) -> Self {
        let hours = seconds / 3600;
        let minutes = (seconds - (hours * 3600)) / 60;
        let seconds = seconds - (hours * 3600) - (minutes * 60);
        Self {
            hours,
            minutes,
            seconds,
        }
    }

    fn to_string(&self) -> String {
        format!("{}{}{}", 
            if self.hours > 0 { format!("{} hours ", self.hours) } else { "".to_string() }, 
            if self.minutes > 0 { format!("{} minutes ", self.minutes) } else { "".to_string() },
            if self.seconds > 0 { format!("{} seconds ", self.seconds) } else { "".to_string() },
        )
    }
}

impl From<&Message> for SubCommand {
    fn from(message: &Message) -> Self {
        let subcommand = message.content.replace("cc remind me", "");
        let r_in_seconds = Regex::new(r" in (?P<seconds>\d+) seconds to (?P<what>.*)").unwrap();
        let r_in_minutes = Regex::new(r" in (?P<minutes>\d+) minutes to (?P<what>.*)").unwrap();
        let r_in_hours = Regex::new(r" in (?P<hours>\d+) hours to (?P<what>.*)").unwrap();
        let r_at = Regex::new(r" at (?P<time>\d+:\d+) (?P<timezone>\w+) to (?P<what>.*)").unwrap();

        if r_in_seconds.is_match(&subcommand) {
            let caps = r_in_seconds.captures(&subcommand).unwrap();
            let seconds = caps.name("seconds").unwrap().as_str().parse::<u64>().unwrap();
            let what = caps.name("what").unwrap().as_str();
            trace!("seconds: {}, what: {}", seconds, what);
            return SubCommand::RemindIn(RemindIn {
                seconds,
                what: what.to_string(),
            });
        } else if r_in_minutes.is_match(&subcommand) {
            let caps = r_in_minutes.captures(&subcommand).unwrap();
            let minutes = caps.name("minutes").unwrap().as_str().parse::<u64>().unwrap();
            let what = caps.name("what").unwrap().as_str();
            trace!("minutes: {}, what: {}", minutes, what);
            return SubCommand::RemindIn(RemindIn {
                seconds: minutes * 60,
                what: what.to_string(),
            });
        } else if r_in_hours.is_match(&subcommand) {
            let caps = r_in_hours.captures(&subcommand).unwrap();
            let hours = caps.name("hours").unwrap().as_str().parse::<u64>().unwrap();
            let what = caps.name("what").unwrap().as_str();
            trace!("hours: {}, what: {}", hours, what);
            return SubCommand::RemindIn(RemindIn {
                seconds: hours * 60 * 60,
                what: what.to_string(),
            });
        } else if r_at.is_match(&subcommand) {
            let caps = r_at.captures(&subcommand).unwrap();
            let time = caps.name("time").unwrap().as_str();
            let timezone = caps.name("timezone").unwrap().as_str();
            let what = caps.name("what").unwrap().as_str();
            trace!("time: {}, timezone: {}, what: {}", time, timezone, what);
            return SubCommand::RemindAt(RemindAt {
                time: time.to_string(),
                timezone: timezone.to_string(),
                what: what.to_string(),
            });
        }
        SubCommand::Help
    }
}

impl RemindCommand {
    pub async fn _help(&self, message: &Message, discord: Arc<Discord>) -> Result<Message, CommandError> {
        let response = format!("Remind me Commands: \n{}", self.help().join("\n"));
        let pc = discord.create_private_channel(message.author.id)?;
        discord.send_message(pc.id, &response, "", false).map_err(|e| e.into())
    }

    pub async fn remind_in(&self, r_in: RemindIn, message: &Message, discord: Arc<Discord>) -> Result<Message, CommandError> {
        let rr_in = r_in.clone();
        let author_id = message.author.id.clone();
        let channel_id = message.channel_id;
        let dd = discord.clone();
        tokio::spawn(async move {
            sleep(Duration::from_secs(rr_in.seconds)).await;
            let response = format!("Hi <@{}>, you asked me to remind you to {}", author_id,  rr_in.what);
            let _ = dd.send_message(channel_id, &response, "", false);
        });

        let response = format!("<@{}>, I will remind you in {}to {}", message.author.id, RemTime::from_seconds(r_in.seconds).to_string(), r_in.what);
        discord.send_message(message.channel_id, &response, "", false).map_err(|e| e.into())
    }

    pub async fn remind_at(&self, at: RemindAt, message: &Message, discord: Arc<Discord>) -> Result<Message, CommandError> {
        let timezone = match at.timezone.to_lowercase().as_str() {
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
        trace!("now    : {}", now.format("%Y-%m-%d %H:%M:%S %z"));
        let timestr = &format!("{} {}:00 {}", now.format("%Y-%m-%d"), at.time, now.format("%z"));
        trace!("timestr: {}", timestr);
        let time: DateTime<Tz> = DateTime::parse_from_str(timestr, "%Y-%m-%d %H:%M:%S %z")?.with_timezone(&tz);
        trace!("time: {}", time);
        if (time - now).num_seconds() < 0 {
            let response = format!("Hi <@{}>, you asked me to remind you at {} {} to {}, but that time has already passed", message.author.id, at.time, at.timezone, at.what);
            return discord.send_message(message.channel_id, &response, "", false).map_err(|e| e.into());
        }
        let diff = time - now;
        let millis = diff.num_milliseconds();
        let dd = discord.clone();
        let aat = at.clone();
        let author_id = message.author.id.clone();
        let channel_id = message.channel_id;        
        tokio::spawn(async move {
            sleep(Duration::from_millis(millis as u64)).await;
            let response = format!("Hi <@{}>, you asked me to remind you to {}", author_id,  aat.what);
            let _ = dd.send_message(channel_id, &response, "", false);
        });

        let response = format!("<@{}>, I will remind you at {} {} to {}", message.author.id, at.time, at.timezone, at.what);
        discord.send_message(message.channel_id, &response, "", false).map_err(|e| e.into())
    }
}

#[async_trait]
impl Responder for RemindCommand {
    type Config = RemindConfig;
    fn new(config: RemindConfig) -> RemindCommand {
        RemindCommand { 
            matches: vec![
                "remind me",
            ],
            _config: config,
        }
    }

    async fn respond(&self, message: &Message, discord: Arc<Discord>, _db: Database) -> Result<Message, CommandError> {
        match SubCommand::from(message) {
            SubCommand::RemindAt(r_at) => self.remind_at(r_at, message, discord).await,
            SubCommand::RemindIn(r_in) => self.remind_in(r_in, message, discord).await,
            SubCommand::Help => self._help(message, discord).await,
        }
    }
}