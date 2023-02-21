use discord::{model::Message, Discord, Result};
use rand::seq::SliceRandom;
use crate::Config;

use super::ChatCommand;

pub struct LoveCommand {
    matches: Vec<&'static str>,
    responses: Vec<&'static str>
}

impl LoveCommand {
    pub fn new() -> LoveCommand {
        LoveCommand { 
            matches: vec![
                "love you",
                "❤️",
            ],
            responses: vec![
                "i love you too!! ❤️❤️❤️",
                "this is a bit awkward... i really do like you but i think i am in love with ... hmm i rather not say. sorry 😿",
                "I saw that you were perfect, and so I loved you. Then I saw that you were not perfect and I loved you even more.",
                "You know you’re in love when you can’t fall asleep because reality is finally better than your dreams. Love you too!",
                "Love is that condition in which the happiness of another person is essential to your own. So what would make you happy?",
                "I need you like a heart needs a beat.",
                "I am who I am because of you. You are every reason, every hope, and every dream I’ve ever had.",
                "If I had a flower for every time I thought of you... I could walk through my garden forever.",
                "It’s always better when we’re together.",
                "I love you for all that you are, all that you have been and all that you will be.",
            ]
        }
    }
    pub fn respond(&self, message: &Message, discord: &Discord) -> Result<Message> {
        discord.send_message(message.channel_id, &format!("<@{}>, {}", message.author.id, self.responses.choose(&mut rand::thread_rng()).unwrap()), "", false)
    }
}

impl ChatCommand for LoveCommand {
    fn matches(&self, message: &str) -> bool {
        self.matches.iter().any(|m| message == *m)
    }

    fn handle(&self, message: &Message, discord: &Discord, _config: &Config) -> Result<Message> {
        self.respond(message, discord)
    }
}