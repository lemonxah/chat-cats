use async_trait::async_trait;
use discord::{model::Message, Discord};
use macros::ChatCommand;
use rand::{seq::SliceRandom, rngs::ThreadRng};
use std::result::Result;
use regex::Regex;
use crate::{Config, storage};
use mongodb::Database;
use super::{ChatCommand, CommandError};

#[derive(ChatCommand)]
pub struct SlapCommand {
    matches: Vec<&'static str>,
    actions: Vec<&'static str>,
    objects: Vec<&'static str>,
    body_parts: Vec<&'static str>,
    results: Vec<&'static str>,
}

impl SlapCommand {
    pub fn new() -> SlapCommand {
        SlapCommand { 
            matches: vec![
                "please slap",
                "slap",
            ],
            actions: vec![
                "slaps",
                "pokes",
                "tickles",
                "strokes",
                "boops",
                "touches",
                "throws",
                "attacks",
                "backhands",
                "shurikens",
                "hadoukens",
                "kamehamehas",
                "FUS ROH DAHs",
            ],
            objects: vec![
                "a wet noodle",
                "a smelly fish",
                "a piece of cake",
                "a bag of angry cats",
                "a bag of howling foxes",
                "a fox tail attached to an angry fox",
                "a slice of pineapple pizza",
                "an agry pink chihuahua",
                "a 4 hour loadshedding",
                "a pink flamingo",
                "a giganotosaurus",
                "a raging pikachu",
            ],
            body_parts: vec![
                "across the face",
                "on the hand",
                "between the eyes",
                "under the feet",
                "on the nose",
                "in the nostril",
                "on the bum bum",
                "on the back",
            ],
            results: vec![
                "it is very effective",
                "<@> brushes it off like nothing",
                "it is not effective at all",
                "<@> stumbles",
                "<@> faints",
                "<@> runs around screaming about bottled water",
                "<@> sneezes",
                "<@> laughs",
                "<@> starts a monolouge on how to exact revenge for this outrage",
                "<@> releases the inner villain",
                "<@> screams MORTAL COMBAT!",
                "<@> just gets obliterated",
            ]
        }
    }

    pub async fn respond(&self, message: &Message, discord: &Discord, db: Database) -> Result<Message, CommandError> {
        let oponent_regex = Regex::new("<@(\\d+?)>")?;
        let oponent_match = oponent_regex.find(&message.content);
        if let Some(opm) = oponent_match {
            let oponent = &message.content[opm.start()..opm.end()];
            let response = {
                let mut rng: ThreadRng = rand::thread_rng();
                let action = *self.actions.choose(&mut rng).unwrap();
                let object = *self.objects.choose(&mut rng).unwrap();
                let body_part = *self.body_parts.choose(&mut rng).unwrap();
                let result = *self.results.choose(&mut rng).unwrap();
                let result = result.replace("<@>", oponent);
                format!("<@{}> {} {} with {} {} and {}", message.author.id, action, oponent, object, body_part, result)
            };
            if !response.ends_with("brushes it off like nothing") && !response.ends_with("it is not effective at all") {
                // slap counts as valid and is recorded
                let _slaps = storage::slaps::slap(message.author.id.0 as i64, oponent[2..oponent.len()-2].parse::<i64>().unwrap(), db).await?;
            }
            discord.send_message(message.channel_id, &response, "", false).map_err(|e| e.into())
        } else {
            discord.send_message(message.channel_id, "You have to @ the person you want to slap", "", false).map_err(|e| e.into())
        }
    }
}