use twitch_irc::message::TwitchUserBasics;
use async_trait::async_trait;
use crate::{bot::{Bot, GlobalBotState}, util};

pub struct Votes (i32, i32);

pub struct State {
    pub is_counting: bool,
    pub voting_box: Votes,
    pub who_voted: Vec<String>,
    pub reset_timestamp: i64,
    pub bot_is_enabled: bool,
}

impl State {
    pub fn to_string(&self) -> String {
        let strings = [
            self.voting_box.0.to_string(), 
            " voted yes, ".to_owned(), 
            self.voting_box.1.to_string(), 
            " voted no!".to_owned()
        ];
        strings.concat()
    }

    pub fn reset(&mut self) {
        self.is_counting = true;
        self.voting_box = Votes(0,0);
        self.who_voted = Vec::new();
        self.reset_timestamp = chrono::offset::Local::now().timestamp_millis();
    }

    pub fn stop_counting(&mut self) {
        self.is_counting = false;
    }

    pub fn add_vote(&mut self, amount: i32, to: i8, voter: &TwitchUserBasics) {
        match to {
            0 => {
                self.voting_box.0 = self.voting_box.0 + amount;
            }
            1 => {
                self.voting_box.1 = self.voting_box.1 + amount;
            }
            _ => {}
        }
        self.who_voted.push(voter.id.clone())
    }

    pub fn can_vote(&self, voter: &TwitchUserBasics) -> bool {
        self.is_counting && !self.who_voted.contains(&voter.id)
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            is_counting: false,
            voting_box: Votes(0,0),
            who_voted: Vec::new(),
            reset_timestamp: chrono::offset::Local::now().timestamp_millis(),
            bot_is_enabled: true,
        }
    }
}

pub struct VoteBot {
    pub state: State,
}

impl Default for VoteBot {
    fn default() -> Self {
        Self { state: Default::default() }
    }
}

#[async_trait]
impl Bot for VoteBot {
    fn is_enabled(&mut self) -> bool {
        self.state.bot_is_enabled
    }

    async fn handle_message(&mut self, global_state: &GlobalBotState, client: &twitch_irc::TwitchIRCClient<twitch_irc::SecureTCPTransport, twitch_irc::login::StaticLoginCredentials>, msg: &twitch_irc::message::PrivmsgMessage) {
        match msg.message_text.to_uppercase().as_str() {
            "1" | "YES" => {
                if self.state.can_vote(&msg.sender) {
                    self.state.add_vote(1, 0, &msg.sender);
                }
            }
            "2" | "NO" => {
                if self.state.can_vote(&msg.sender) {
                    self.state.add_vote(1, 1, &msg.sender);
                }
            }
            "!RESULTS_VOTES" => {
                if util::is_mod(&msg.sender) {
                    self.state.stop_counting();
                    let message = [
                        "@".to_owned(),
                        msg.sender.name.clone().to_owned(),
                        " ".to_owned(),
                        self.state.to_string()
                    ].concat();
                    client.say(global_state.channel_name.to_owned(), message).await.unwrap();
                }
            }
            "!RESET_VOTES" => {
                if util::is_mod(&msg.sender) {
                    self.state.reset();
                    client.say(
                        global_state.channel_name.to_owned(), 
                        "Reset votes! Vote Yes with 1 and No with 2!".to_owned()
                    ).await.unwrap();
                }
            }
            "!STOP_VOTES" => {
                if util::is_mod(&msg.sender) {
                    self.state.stop_counting();
                    client.say(
                        global_state.channel_name.to_owned(), 
                        "Stopped counting!".to_owned()
                    ).await.unwrap();
                }
            }
            _ => {}
        }
    }
}