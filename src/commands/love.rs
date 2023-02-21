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
                "<Insert loving message for the chatter that loves you here>",
                "You know that tingly little feeling you get when you like someone? That is your common sense leaving your body.",
                "I want someone who will look at me the same way I look at chocolate cake.",
                "I love you with all my belly. I would say heart, but my belly is bigger.",
                "You want to know who I'm in love with? Read the first word again.",
                "I love you so much I'd fight a bear for you. Well not a grizzly bear because they have claws, and not a panda bear because they know Kung Fu. But a care bear, I'd definitely fight a care bear for you.",
                "Will you lend me a kiss? I promise to give it back.",
                "Forget the butterflies, I feel the entire zoo in my stomach when I'm with you!",
                "Good thing I brought my library card because I'm totally checking you out.",
                "I love you more than coffee, but please don't make me prove it.",
                "I want to be the reason you look down at your phone and smile. Then walk into a pole.",
                "Besides chocolate, you're my favorite.",
                "I think you are suffering from a lack of vitamin ME.",
                "I love you even when I'm really, really hungry.",
                "I love you more than cake, and I really love cake.",
                "Relationships are like a walk in the park. Jurassic Park.",
                "I love you enough to make our iPhone-Samsung relationship work.",
                "All you need is love. But a little chocolate now and then doesn't hurt.",
                "Before i answer, please use a computer with slow Internet service so that I can see who you really are.",
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