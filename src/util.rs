use twitch_irc::message::TwitchUserBasics;

pub fn is_mod(sender: &TwitchUserBasics) -> bool {
    sender.id == "207883858"
}

#[derive(Clone)]
pub struct GlobalState {
    pub bot_name: String,
    pub channel_name: String,
}