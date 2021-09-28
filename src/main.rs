use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;

struct Votes (i32, i32);

fn is_mod(username: &String) -> bool {
    username == "onelikeandidie"
}

struct State {
    is_counting: bool,
    voting_box: Votes
}

impl State {
    fn to_string(&self) -> String{
        let strings = [self.voting_box.0.to_string(), " voted yes, ".to_owned(), self.voting_box.1.to_string(), " voted no!".to_owned()];
        strings.concat()
    }

    fn add_vote(&mut self, amount: i32, to: i8) {
        match to {
            0 => {
                self.voting_box.0 = self.voting_box.0 + amount;
            }
            1 => {
                self.voting_box.1 = self.voting_box.1 + amount;
            }
            _ => {}
        }
    }
}

#[tokio::main]
pub async fn main() {
    let login_name = "onelikeandishutdown".to_owned();
    let oauth_token = "5dqam1yud0u6f962xblrkzfy8pid1n".to_owned();
    let channel_login = "onelikeandidie";
    
    let config = ClientConfig::new_simple(
        StaticLoginCredentials::new(login_name, Some(oauth_token))
    );

    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);

    let mut state: State = State{is_counting: true, voting_box: Votes(0,0)};

    // first thing you should do: start consuming incoming messages,
    // otherwise they will back up.
    let thread_client = client.clone();
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            //println!("Received message: {:?}", message);
            match message {
                ServerMessage::Privmsg(msg) => {
                    match msg.message_text.as_str() {
                        "1" => {
                            if state.is_counting {
                                state.add_vote(1, 0);
                            }
                        }
                        "2" => {
                            if state.is_counting {
                                state.add_vote(1, 1);
                            }
                        }
                        "!results" => {
                            if is_mod(&msg.sender.name) {
                                state.is_counting = false;
                                let message = [
                                    "@".to_owned(), 
                                    msg.sender.name.clone().to_owned(), 
                                    " ".to_owned(), 
                                    state.to_string()
                                    ].concat();
                                thread_client.say(channel_login.to_owned(), message).await.unwrap();
                            }
                        }
                        "!reset" => {
                            if is_mod(&msg.sender.name) {
                                state.is_counting = true;
                                state.voting_box = Votes(0,0);
                                thread_client.say(channel_login.to_owned(), "Reset votes!".to_owned()).await.unwrap();
                            }
                        }
                        "!stop" => {
                            if is_mod(&msg.sender.name) {
                                state.is_counting = false;
                                thread_client.say(channel_login.to_owned(), "Stopped counting!".to_owned()).await.unwrap();
                            }
                        }
                        _ => {}
                    }
                },
                //ServerMessage::ClearChat(_) => todo!(),
                //ServerMessage::ClearMsg(_) => todo!(),
                //ServerMessage::GlobalUserState(_) => todo!(),
                //ServerMessage::HostTarget(_) => todo!(),
                //ServerMessage::Join(_) => todo!(),
                //ServerMessage::Notice(_) => todo!(),
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
    client.join(channel_login.to_owned());

    // keep the tokio executor alive.
    // If you return instead of waiting the background task will exit.
    join_handle.await.unwrap();
}