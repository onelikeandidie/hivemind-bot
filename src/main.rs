use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;

mod bot;
mod vote_bot;
mod league_bot;
mod util;

use crate::bot::Bot;

#[tokio::main]
pub async fn main() {
    let oauth_token = "token".to_owned();
    let bot_name = "onelikeandishutdown".to_owned();
    let channel_name = "onelikeandidie".to_owned();

    let state = util::GlobalState { 
        bot_name: bot_name.clone(),
        channel_name: channel_name.clone()
    };
    
    let config = ClientConfig::new_simple(
        StaticLoginCredentials::new(state.bot_name.clone(), Some(oauth_token))
    );

    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let mut vote_bot_instance = vote_bot::VoteBot::default();
    let mut league_bot_instance = league_bot::LeagueBot::default();

    // first thing you should do: start consuming incoming messages,
    // otherwise they will back up.
    let thread_client = client.clone();
    let thread_state = state.clone();
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            //println!("Received message: {:?}", message);
            match message {
                ServerMessage::Privmsg(msg) => {
                    vote_bot_instance.handle_message(&thread_state, &thread_client, &msg).await;
                    league_bot_instance.handle_message(&thread_state, &thread_client, &msg).await;
                },
                //ServerMessage::ClearChat(_) => todo!(),
                //ServerMessage::ClearMsg(_) => todo!(),
                //ServerMessage::GlobalUserState(_) => todo!(),
                //ServerMessage::HostTarget(_) => todo!(),
                //ServerMessage::Join(_) => todo!(),
                ServerMessage::Notice(msg) => {
                    println!("Recieved notice: {:?}", msg.message_text);
                    if msg.message_text == "Login authentication failed" {
                        thread_client.part(thread_state.channel_name.clone());
                        incoming_messages.close();
                    }
                },
                //ServerMessage::Part(_) => todo!(),
                //ServerMessage::Ping(_) => todo!(),
                //ServerMessage::Pong(_) => todo!(),
                //ServerMessage::Reconnect(_) => todo!(),
                //ServerMessage::RoomState(_) => todo!(),
                //ServerMessage::UserNotice(_) => todo!(),
                //ServerMessage::UserState(_) => todo!(),
                //ServerMessage::Whisper(_) => todo!(),
                //ServerMessage::Generic(_) => todo!(),
                _ => {}
            }
        }
    });

    // join a channel
    client.join(channel_name.clone().to_owned());

    // keep the tokio executor alive.
    // If you return instead of waiting the background task will exit.
    join_handle.await.unwrap();
}