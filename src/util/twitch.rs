use twitch_irc::message::{PrivmsgMessage};

pub fn is_mod(msg: &PrivmsgMessage/* , sender: &TwitchUserBasics */) -> bool {
    //sender.id == "207883858";
    for badge in &msg.badges {
        if badge.name == "moderator" || badge.name == "broadcaster" {
            return true;
        }
    }
    false
}