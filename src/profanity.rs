use std::sync::Arc;

use rand::{thread_rng, Rng};
use regex::Regex;
use discord::model::{ChannelId, MessageId, UserId};
use discord::Discord;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ProfanityConfig {
    pub bad_words: Vec<String>,
    pub replacements: Vec<String>,
}

pub fn profanity_filter(message_id: MessageId, author_id: UserId, content: String, channel_id: ChannelId, config: &ProfanityConfig, discord: Arc<Discord>) {
    let mut rng = thread_rng();
    let n = rng.gen_range(0..config.replacements.len()-1);
    let re_string = format!("(?i)({})", config.bad_words.join("|"));
    let re = Regex::new(&re_string).unwrap();
    if re.is_match(&content) {
        let new_text = re.replace_all(&content, &config.replacements[n]);
        let text = format!("I am sure <@{}> meant to say: {}", author_id, new_text.to_string());
        let _ = discord.delete_message(channel_id, message_id);
        let _ = discord.send_message(channel_id, &text, "", false);
    }
}