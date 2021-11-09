use serde::{Deserialize, Serialize};
use twitch_irc::message::TwitchUserBasics;

pub fn is_mod(sender: &TwitchUserBasics) -> bool {
    sender.id == "207883858"
}

#[derive(Clone)]
pub struct GlobalState {
    pub bot_name: String,
    pub channel_name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub oauth_token: String,
    pub bot_name: String,
    pub channel_name: String,
}