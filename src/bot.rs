use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::PrivmsgMessage;
use async_trait::async_trait;

use crate::util;

#[async_trait]
pub trait Bot {
    fn is_enabled(&mut self) -> bool;
    async fn handle_message(&mut self, global_state: &util::GlobalState, client: &TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>, msg: &PrivmsgMessage);
    async fn update(&mut self, global_state: &util::GlobalState, client: &TwitchIRCClient<SecureTCPTransport, StaticLoginCredentials>);
}