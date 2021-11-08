use twitch_irc::message::TwitchUserBasics;

pub fn is_mod(sender: &TwitchUserBasics) -> bool {
    sender.id == "207883858"
}