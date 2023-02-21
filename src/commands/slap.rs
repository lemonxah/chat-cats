use discord::{model::Message, Discord, Result};
use rand::{seq::SliceRandom, rngs::ThreadRng};
use regex::Regex;
use crate::Config;

use super::ChatCommand;

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
    pub fn respond(&self, message: &Message, discord: &Discord, rng: &mut ThreadRng) -> Result<Message> {
        let oponent_regex = Regex::new("<@\\d+?>").unwrap();
        let oponent_match = oponent_regex.find(&message.content);
        if let Some(opm) = oponent_match {
            let oponent = &message.content[opm.start()..opm.end()];
            let action = *self.actions.choose(rng).unwrap();
            let object = *self.objects.choose(rng).unwrap();
            let body_part = *self.body_parts.choose(rng).unwrap();
            let result = *self.results.choose(rng).unwrap();
            let result = result.replace("<@>", oponent);
            let response = format!("<@{}> {} {} with {} {} and {}", message.author.id, action, oponent, object, body_part, result);
            discord.send_message(message.channel_id, &response, "", false)
        } else {
            discord.send_message(message.channel_id, "You have to @ the person you want to slap", "", false)
        }
    }
}

impl ChatCommand for SlapCommand {
    fn matches(&self, message: &str) -> bool {
        self.matches.iter().any(|m| message.starts_with(m))
    }

    fn handle(&self, message: &Message, discord: &Discord, _config: &Config, rng: &mut ThreadRng) -> Result<Message> {
        self.respond(message, discord, rng)
    }
}