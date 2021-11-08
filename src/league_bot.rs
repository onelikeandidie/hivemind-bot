use std::{thread, time::{Duration}};
use tokio::{sync::oneshot, time::timeout};
use twitch_irc::message::{PrivmsgMessage, TwitchUserBasics};
use async_trait::async_trait;
use crate::{bot::{Bot, GlobalBotState}, util};

// Q, W, E, R
pub struct Votes (i32, i32, i32, i32);

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
            " Q, ".to_owned(),
            self.voting_box.1.to_string(),
            " W, ".to_owned(),
            self.voting_box.2.to_string(),
            " E, ".to_owned(),
            self.voting_box.3.to_string(),
            " R!".to_owned()
        ];
        strings.concat()
    }

    pub fn reset(&mut self) {
        self.is_counting = true;
        self.voting_box = Votes(0, 0, 0, 0);
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
            2 => {
                self.voting_box.2 = self.voting_box.2 + amount;
            }
            3 => {
                self.voting_box.3 = self.voting_box.3 + amount;
            }
            _ => {}
        }
        self.who_voted.push(voter.id.clone())
    }

    pub fn can_vote(&self, voter: &TwitchUserBasics) -> bool {
        self.is_counting && !self.who_voted.contains(&voter.id)
    }

    pub fn get_results_message(&self, msg: &PrivmsgMessage) -> String {
        [
            "@".to_owned(),
            msg.sender.name.clone().to_owned(),
            " ".to_owned(),
            self.to_string()
        ].concat()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            is_counting: false,
            voting_box: Votes(0, 0, 0, 0),
            who_voted: Vec::new(),
            reset_timestamp: chrono::offset::Local::now().timestamp_millis(),
            bot_is_enabled: true,
        }
    }
}

pub struct LeagueBot {
    pub state: State,
}

impl Default for LeagueBot {
    fn default() -> Self {
        Self { state: Default::default() }
    }
}

#[async_trait]
impl Bot for LeagueBot {
    fn is_enabled(&mut self) -> bool {
        self.state.bot_is_enabled
    }

    async fn handle_message(&mut self, global_state: &GlobalBotState, client: &twitch_irc::TwitchIRCClient<twitch_irc::SecureTCPTransport, twitch_irc::login::StaticLoginCredentials>, msg: &PrivmsgMessage) {
        match msg.message_text.to_uppercase().as_str() {
            "Q" | "1" => {
                if self.state.can_vote(&msg.sender) {
                    self.state.add_vote(1, 0, &msg.sender);
                }
            }
            "W" | "2" => {
                if self.state.can_vote(&msg.sender) {
                    self.state.add_vote(1, 1, &msg.sender);
                }
            }
            "E" | "3" => {
                if self.state.can_vote(&msg.sender) {
                    self.state.add_vote(1, 2, &msg.sender);
                }
            }
            "R" | "4" => {
                if self.state.can_vote(&msg.sender) {
                    self.state.add_vote(1, 3, &msg.sender);
                }
            }
            "!RESULTS_LEAGUE" => {
                if util::is_mod(&msg.sender) {
                    self.state.stop_counting();
                    let message = self.state.get_results_message(msg);
                    client.say(global_state.channel_name.to_owned(), message).await.unwrap();
                }
            }
            "!RESET_LEAGUE" => {
                if util::is_mod(&msg.sender) {
                    self.state.reset();
                    client.say(
                        global_state.channel_name.to_owned(), 
                        "Reset votes! Vote Q, W, E, R to level an ability!".to_owned()
                    ).await.unwrap();
                }
            }
            "!UP_LEAGUE" => {
                if util::is_mod(&msg.sender) {
                    // WARNING: THIS DOESN'T WORK BECAUSE THREAD IS ASLEEP!
                    // Reset the votes
                    self.state.reset();
                    client.say(
                        global_state.channel_name.to_owned(), 
                        "Vote Q, W, E, R to level an ability! You have 10 seconds!".to_owned()
                    ).await.unwrap();

                    // Wait 10 Seconds
                    let sleep_duration = Duration::from_secs(10);
                    thread::sleep(sleep_duration);

                    // Stop counting and get results
                    self.state.stop_counting();
                    let message = self.state.get_results_message(msg);
                    client.say(global_state.channel_name.to_owned(), message).await.unwrap();
                }
            }
            "!STOP_LEAGUE" => {
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