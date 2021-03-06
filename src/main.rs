use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tokio::time::interval;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;

mod bots;
mod util;

use crate::util::bot::{Bot, Config, GlobalState};
use crate::bots::{league_bot, vote_bot};

#[tokio::main]
pub async fn main() {
    let bot_config: Config;
    match tokio::fs::read_to_string("./config.toml").await {
        Ok(bot_config_file) => {
            bot_config = toml::from_str(&bot_config_file).unwrap();
        },
        Err(_) => panic!("No config provided or format is not compliant.")
    };

    let oauth_token = bot_config.oauth_token.to_owned();
    let bot_name = bot_config.bot_name.to_owned();
    let channel_name = bot_config.channel_name.to_owned();

    let state = GlobalState { 
        bot_name: bot_name.clone(),
        channel_name: channel_name.clone()
    };

    let config = ClientConfig::new_simple(
        StaticLoginCredentials::new(state.bot_name.clone(), Some(oauth_token))
    );

    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    // Create the bots
    // Maybe there's a better way to store pointers like this?
    let vote_bot_box = Arc::new(Mutex::new(vote_bot::VoteBot::default()));
    let league_bot_box = Arc::new(Mutex::new(league_bot::LeagueBot::default()));

    let (tx, mut rx) = mpsc::channel(100);

    // First thread, consuming messages from Twitch. This is a separate
    // thread because they would clog up if the thread is blocked
    let thread_client = client.clone();
    let thread_state = state.clone();
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            //println!("Received message: {:?}", message);
            match message {
                ServerMessage::Privmsg(msg) => {
                    tx.send(msg.clone()).await.unwrap();
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

    // Second thread with bot message handling
    let thread_client = client.clone();
    let thread_state = state.clone();
    let vb_arc = vote_bot_box.clone();
    let lb_arc = league_bot_box.clone();
    let message_handler_handle = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            // Upstream messages to bots
            vb_arc.lock().await.handle_message(&thread_state, &thread_client, &msg).await;
            lb_arc.lock().await.handle_message(&thread_state, &thread_client, &msg).await;
        }
    });

    // Third thread with bot updating every 2 seconds
    let thread_client = client.clone();
    let thread_state = state.clone();
    let vb_arc = vote_bot_box.clone();
    let lb_arc = league_bot_box.clone();
    let updater_handle = tokio::spawn(async move {
        let mut it = interval(Duration::from_secs(1));
        // Update loop, waits for the tick
        loop {
            it.tick().await;
            // Update bots
            vb_arc.lock().await.update(&thread_state, &thread_client).await;
            lb_arc.lock().await.update(&thread_state, &thread_client).await;
        }
    });

    // join a channel
    client.join(channel_name.clone().to_owned());

    // keep the tokio executor alive.
    // If you return instead of waiting the background task will exit.
    join_handle.await.unwrap();
    message_handler_handle.await.unwrap();
    updater_handle.await.unwrap();
}