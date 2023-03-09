use rand::{thread_rng, Rng};
use regex::Regex;
use discord::model::Message;
use discord::Discord;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ProfanityConfig {
    pub bad_words: Vec<String>,
    pub replacements: Vec<String>,
}

pub fn profanity_filter(message: &Message, config: &ProfanityConfig, discord: &Discord) {
    let mut rng = thread_rng();
    let n = rng.gen_range(0..config.replacements.len()-1);
    let re_string = format!("(?i)({})", config.bad_words.join("|"));
    let re = Regex::new(&re_string).unwrap();
    if re.is_match(&message.content) {
        let new_text = re.replace_all(&message.content, &config.replacements[n]);
        let text = format!("I am sure <@{}> meant to say: {}", message.author.id, new_text.to_string());
        let _ = discord.delete_message(message.channel_id, message.id);
        let _ = discord.send_message(message.channel_id, &text, "", false);
    }
}