use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::PrivmsgMessage;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

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
    pub draw_window: bool,
}

#[async_trait]
pub trait Bot {
    fn is_enabled(&mut self) -> bool;
    async fn handle_message(&mut self, global_state: &GlobalState, client: &TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>, msg: &PrivmsgMessage);
    async fn update(&mut self, global_state: &GlobalState, client: &TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>);
}