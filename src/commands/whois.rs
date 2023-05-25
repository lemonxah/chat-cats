use async_trait::async_trait;
use discord::{model::Message, Discord};
use macros::ChatCommand;
use std::{result::Result, sync::Arc};
use mongodb::Database;
use regex::Regex;
use crate::storage::whois;
use super::{ChatCommand, CommandError, Responder, HelpCommands};

#[derive(ChatCommand)]
pub struct WhoIsCommand {
    matches: Vec<&'static str>,
}

impl HelpCommands for WhoIsCommand {
    fn help() -> Vec<&'static str> {
        vec![
            "   who is @target - Responds with saved message for target",
            "       Example: cc WhoIs @chat-cats",
            "   @target is <message> - Saves message for target",
            "       Example: cc @chat-cats is a cool bot",
        ]
    }
}

#[async_trait]
impl Responder for WhoIsCommand {
    type Config = ();
    fn new(_config: Self::Config) -> WhoIsCommand {
        WhoIsCommand { 
            matches: vec![
                "who is",
                "<@.+?> is",
            ],
        }
    }
    async fn respond(&self, message: &Message, discord: Arc<Discord>, db: Database) -> Result<Message, CommandError> {
        let r_who_is = Regex::new(r"cc who is (?P<user>.+?)$").unwrap();
        let r_is = Regex::new(r"cc (?P<user>.+?) is (?P<message>.+?)").unwrap();
        if r_who_is.is_match(&message.content) {
            let caps = r_who_is.captures(message.content.as_str()).unwrap();
            let user_str = caps.name("user").unwrap().as_str();
            let target_regex = Regex::new("<@(\\d+?)>").unwrap();
            if let Some(target_match) = target_regex.find(user_str) {
                let userid_str = user_str[target_match.start()..target_match.end()].to_string();
                log::info!("userid_str: {}", userid_str);
                let userid: i64 = userid_str.parse::<i64>().unwrap();
                let is_message = whois::whois(userid, db).await;
                if is_message.is_ok() {
                    let response = {
                        format!("<@{}> is {} ", userid, is_message.unwrap().message)
                    };
                    return discord.send_message(message.channel_id, &response, "", false).map_err(|e| e.into())
                } else {
                    let response = {
                        format!("sorry i do not know <@{}> ", userid)
                    };
                    return discord.send_message(message.channel_id, &response, "", false).map_err(|e| e.into())
                }
            }
        }
        if r_is.is_match(&message.content) {
            let caps = r_is.captures(message.content.as_str()).unwrap();
            let user_str = caps.name("user").unwrap().as_str();
            let message_str = caps.name("message").unwrap().as_str();
            let target_regex = Regex::new("<@(\\d+?)>").unwrap();
            if let Some(target_match) = target_regex.find(user_str) {
                let userid: i64 = user_str[target_match.start()..target_match.end()].to_string().parse::<i64>().unwrap();
                let is_message = whois::is(userid, message_str.to_owned(), db).await;
                if is_message.is_ok() {
                    let response = {
                        format!("i now know <@{}> ", userid)
                    };
                    return discord.send_message(message.channel_id, &response, "", false).map_err(|e| e.into())
                } else {
                    let response = {
                        format!("sorry i could not save <@{}> ", userid)
                    };
                    return discord.send_message(message.channel_id, &response, "", false).map_err(|e| e.into())
                }
            }
        }
        let response = {
            format!("Sorry i do not understand that command")
        };
        discord.send_message(message.channel_id, &response, "", false).map_err(|e| e.into())
    }
}
