use std::{fmt::Display, time::Duration};

use inputbot::KeybdKey;
use tokio::time::sleep;
//use std::{thread, time::{Duration}};
use twitch_irc::message::{PrivmsgMessage, TwitchUserBasics};
use async_trait::async_trait;
use crate::{bot::{Bot}, util::{GlobalState, is_mod}};

#[derive(Clone, Copy)]
// Q, W, E, R
pub struct Votes (i32, i32, i32, i32);

impl Votes {
    fn most_voted(& self) -> Option<Poggers> {
        let values = [self.0, self.1, self.2, self.3];
        let max = values.iter().max().unwrap();
        if let Some(index_of_max) = values.iter().position(|&x| x == *max) {
            match index_of_max {
                0 => Some(Poggers::Q),
                1 => Some(Poggers::W),
                2 => Some(Poggers::E),
                3 => Some(Poggers::R),
                _ => None
            }
        } else {
            return None;
        }
    }
}

enum Poggers {
    Q,W,E,R
}

impl Display for Poggers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Poggers::Q => "Q",
            Poggers::W => "W",
            Poggers::E => "E",
            Poggers::R => "R",
        })
    }
}

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

    pub fn get_results_message(&self, msg: Option<&PrivmsgMessage>) -> String {
        if let Some(msg) = msg {
            return [
                "@".to_owned(),
                msg.sender.name.clone().to_owned(),
                " ".to_owned(),
                self.to_string()
            ].concat()
        } else {
            return [
                "@onelikeandidie ".to_owned(),
                self.to_string()
            ].concat()
        }
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

    async fn handle_message(&mut self, global_state: &GlobalState, client: &twitch_irc::TwitchIRCClient<twitch_irc::SecureTCPTransport, twitch_irc::login::StaticLoginCredentials>, msg: &PrivmsgMessage) {
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
                if is_mod(&msg.sender) {
                    self.state.stop_counting();
                    let message = self.state.get_results_message(Some(msg));
                    client.say(global_state.channel_name.to_owned(), message).await.unwrap();
                }
            }
            "!UP_LEAGUE" |
            "!RESET_LEAGUE" => {
                if is_mod(&msg.sender) {
                    self.state.reset();
                    client.say(
                        global_state.channel_name.to_owned(), 
                        "Vote Q, W, E, R to level an ability!".to_owned()
                    ).await.unwrap();
                }
            }
            "!STOP_LEAGUE" => {
                if is_mod(&msg.sender) {
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

    async fn update(&mut self, global_state: &GlobalState, client: &twitch_irc::TwitchIRCClient<twitch_irc::SecureTCPTransport, twitch_irc::login::StaticLoginCredentials>) {
        //println!("League Bot Updated");
        
        // Check if more than 10 seconds have passed since started counting
        let now = chrono::offset::Local::now().timestamp_millis();
        if self.state.is_counting && (now - self.state.reset_timestamp > 10000 /* 10 sec in ms */) {
            self.state.stop_counting();
            let message = self.state.get_results_message(None);
            client.say(global_state.channel_name.to_owned(), message).await.unwrap();
            println!("{}", [
                "[LeagueBot]",
                " ",
                "Finished counting ability votes"
                ].concat());
            
            tokio::spawn(LeagueBot::level_up_ability(self.state.voting_box));
        }
    }
}

impl LeagueBot {
    async fn level_up_ability(votes: Votes) {
        // Calculate most voted for
        if let Some(vote) = votes.most_voted() {
            // Press the upgrade buttons
            let ability_button = match vote {
                Poggers::Q => KeybdKey::QKey,
                Poggers::W => KeybdKey::WKey,
                Poggers::E => KeybdKey::EKey,
                Poggers::R => KeybdKey::RKey,
            };
            KeybdKey::LControlKey.press();
            ability_button.press();
            sleep(Duration::from_millis(50)).await; // This might become a problem
            ability_button.release();
            KeybdKey::LControlKey.release();

            println!("{} {}", [
                "[LeagueBot]",
                " ",
                "Leveled up"
                ].concat(), vote);
        }
    }
}