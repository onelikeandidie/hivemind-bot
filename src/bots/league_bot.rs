use std::{env, fmt::Display, io::Read, time::Duration};
use inputbot::{KeySequence, KeybdKey};
use tokio::time::sleep;
//use std::{thread, time::{Duration}};
use twitch_irc::message::{PrivmsgMessage, TwitchUserBasics};
use async_trait::async_trait;
use crate::util::{bot::{Bot, GlobalState}, twitch::is_mod, league::{LeagueResponse}};

#[derive(Clone, Copy)]
// Q, W, E, R
pub struct Votes (pub i32, pub i32, pub i32, pub i32);

impl Votes {
    pub fn most_voted_index(& self) -> Option<usize> {
        let values = [self.0, self.1, self.2, self.3];
        if values.iter().max() == values.iter().min() {
            return None;
        }
        if let Some(max) = values.iter().max() {
            values.iter().position(|&x| x == *max)
        } else {
            None
        }
    }

    pub fn most_voted(& self) -> Option<Poggers> {
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

impl Default for Votes {
    fn default() -> Self {
        Self(0, 0, 0, 0)
    }
}

pub enum Poggers {
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
    // Voting Related Stuff
    pub is_counting: bool,
    pub voting_box: Votes,
    pub who_voted: Vec<String>,
    pub reset_timestamp: i64,
    pub bot_is_enabled: bool,
    // League Client Related Stuff
    pub http_client: reqwest::Client,
    pub http_client_attempt_connect: bool,
    pub http_client_connected: bool,
    pub url: String,
    // League Votes Related Stuff
    pub last_league_state: Option<LeagueResponse>,
    pub last_request_timestamp: i64,
    pub should_poll_for_level: bool,
    pub force_check_level: bool,
    pub last_level: i32,
    pub ff_counter: i32,
    pub ff_reset_timestamp: i64,
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
    /** Reset the votes and start counting */
    pub fn reset(&mut self) {
        self.is_counting = true;
        self.voting_box = Votes(0, 0, 0, 0);
        self.who_voted = Vec::new();
        self.reset_timestamp = chrono::offset::Local::now().timestamp_millis();
    }
    /** Stop counting */
    pub fn stop_counting(&mut self) {
        self.is_counting = false;
    }
    /** Add a vote to the box, the `to` is the index of the box to add to */
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
    /** Returns true if a user has not voted */
    pub fn can_vote(&self, voter: &TwitchUserBasics) -> bool {
        self.is_counting && !self.who_voted.contains(&voter.id)
    }
    /** 
    Compile a small string reading out the votebox into a message, if a msg is
    provided with `Some(msg)` the command will use the message's sender as the
    user to mention on the resulting message.
    */
    pub fn get_results_message(&self, msg: Option<&PrivmsgMessage>, user_to_mention: Option<&String>) -> String {
        if let Some(msg) = msg {
            return [
                "@".to_owned(),
                msg.sender.name.clone().to_owned(),
                " ".to_owned(),
                self.to_string()
            ].concat()
        } else {
            if let Some(user_to_mention) = user_to_mention {
                return [
                    "@".to_owned(),
                    user_to_mention.to_owned(),
                    self.to_string()
                ].concat()
            } else {
                return self.to_string();
            }
        }
    }
    /** Resets the ff counter and it's associated timestamp */
    pub fn ff_reset(&mut self) {
        self.ff_counter = 0;
        self.ff_reset_timestamp = chrono::offset::Local::now().timestamp_millis();
    }
}

impl Default for State {
    fn default() -> Self {
        // Load RITO GAMES certificate
        let mut buf = Vec::new();
        std::fs::File::open("external/riotgames.pem").unwrap()
            .read_to_end(&mut buf).unwrap();
        let cert = reqwest::Certificate::from_pem(&buf).unwrap();
        // Create a http client that uses the certificate
        let client = reqwest::Client::builder()
            .add_root_certificate(cert)
            .build().unwrap();
        // Parse the URL to the game client end point
        let url = "https://127.0.0.1:2999/liveclientdata/activeplayer".to_owned();

        let now = chrono::offset::Local::now().timestamp_millis();
        Self {
            is_counting: false,
            voting_box: Votes(0, 0, 0, 0),
            who_voted: Vec::new(),
            reset_timestamp: now.clone(),
            bot_is_enabled: true,

            http_client: client,
            http_client_attempt_connect: true,
            http_client_connected: false,
            url: url,

            last_request_timestamp: now.clone(),
            last_league_state: Some(LeagueResponse::default()),
            should_poll_for_level: false,
            force_check_level: true,
            last_level: 0,
            ff_counter: 0,
            ff_reset_timestamp: now.clone(),
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
        let upper = msg.message_text.to_uppercase();
        println!("{:?}", upper.trim().split(" "));
        let message_args = upper.trim().split(" ").collect::<Vec<&str>>();
        match message_args[0] {
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
            "FF" => {
                self.state.ff_counter += 1;
            }
            "!RESULTS_LEAGUE" => {
                if is_mod(&msg) {
                    self.state.stop_counting();
                    let message = self.state.get_results_message(Some(msg), None);
                    client.say(global_state.channel_name.to_owned(), message).await.unwrap();
                }
            }
            "!UP_LEAGUE" |
            "!RESET_LEAGUE" => {
                if is_mod(&msg) {
                    self.state.should_poll_for_level = true;
                }
            }
            "!STOP_LEAGUE" => {
                if is_mod(&msg) {
                    self.state.stop_counting();
                    client.say(
                        global_state.channel_name.to_owned(), 
                        "Stopped counting!".to_owned()
                    ).await.unwrap();
                }
            }
            "!RECONNECT_LEAGUE" => {
                if is_mod(&msg) {
                    self.state.http_client_attempt_connect = true;
                    self.state.force_check_level = true;
                }
            }
            _ => {}
        }
    }

    async fn update(&mut self, global_state: &GlobalState, client: &twitch_irc::TwitchIRCClient<twitch_irc::SecureTCPTransport, twitch_irc::login::StaticLoginCredentials>) {
        // Declare the current time
        let now = chrono::offset::Local::now().timestamp_millis();
        
        // Check the client for automated leveling
        if self.state.http_client_attempt_connect {
            //println!("[LeagueBot] Connecting to the league client");
            self.update_league_client().await;
            //println!("[LeagueBot] Finnish");
        }
        
        // Check if the league client is connected
        if self.state.http_client_connected {
            // Force calculate levelups
            if self.state.force_check_level {
                if let Some(lls) = &self.state.last_league_state {
                    self.state.last_level = lls.abilities.check_used_points();
                    println!("[LeagueBot] Level was force checked, new level: {}", self.state.last_level);
                }
                self.state.force_check_level = false;
            }

            self.check_league_client().await;

            // Check if more than 10 seconds have passed since started counting
            if self.state.is_counting && (now - self.state.reset_timestamp > 10000 /* 10 sec in ms */) {
                self.state.stop_counting();
                let message = self.state.get_results_message(None, Some(&global_state.channel_name.clone().to_owned()));
                client.say(global_state.channel_name.to_owned(), message).await.unwrap();

                match self.state.voting_box.most_voted() {
                    Some(vote) => {
                        println!("[LeagueBot] Finished counting ability votes");
                        tokio::spawn(LeagueBot::level_up_ability(vote));
                        self.state.force_check_level = true;
                    },
                    None => {
                        println!("[LeagueBot] No Votes lol gg vote again");
                        self.state.reset();
                    }
                }
            }

            // Check if a lot of people have voted to ff rather quickly
            if now - self.state.ff_reset_timestamp > 5000 /* 5 sec in ms */ {
                if self.state.ff_counter > 20 {
                    println!("[LeagueBot] Forcing FF vote");
                    tokio::spawn(LeagueBot::try_to_ff());
                    self.state.ff_reset();
                }
            }

            // Check if the bot should poll for level
            if self.state.should_poll_for_level {
                self.state.reset();
                client.say(
                    global_state.channel_name.to_owned(), 
                    "Vote Q, W, E, R to level an ability!".to_owned()
                ).await.unwrap();
                self.state.should_poll_for_level = false;
            }
        }
    }
}

impl LeagueBot {
    /** Attempt to press the Keyboard buttons to level up an ability */
    async fn level_up_ability(vote: Poggers) {
        // Press the upgrade buttons
        let ability_button = match vote {
            Poggers::Q => KeybdKey::QKey,
            Poggers::W => KeybdKey::WKey,
            Poggers::E => KeybdKey::EKey,
            Poggers::R => KeybdKey::RKey,
        };
        KeybdKey::LControlKey.press();
        ability_button.press();
        sleep(Duration::from_millis(20)).await; // This might become a problem
        ability_button.release();
        KeybdKey::LControlKey.release();

        println!("{} {}", [
            "[LeagueBot]",
            " ",
            "Leveled up"
            ].concat(), vote);
    }
    /** Attempt to press the KeySequence to initiate a ff vote */
    async fn try_to_ff() {
        KeybdKey::EnterKey.press();
        KeybdKey::EnterKey.release();
        sleep(Duration::from_millis(20)).await;
        if env::consts::OS == "windows" {
            // 0xBF == Slash key on windows
            KeybdKey::OtherKey(0xBF).press();
            sleep(Duration::from_millis(20)).await;
            KeybdKey::OtherKey(0xBF).release();
        } else {
            // 0x02f == Slash key on linux
            KeybdKey::OtherKey(0x02f).press();
            sleep(Duration::from_millis(20)).await;
            KeybdKey::OtherKey(0x02f).release();
        }
        KeySequence("ff").send();
        sleep(Duration::from_millis(20)).await;
        KeybdKey::EnterKey.press();
        KeybdKey::EnterKey.release();
    }
    /** Update the saved state of the league client */
    async fn update_league_client(&mut self) {
        // Run the casul GET request to the client backend
        match self.state.http_client.get(self.state.url.clone()).send().await {
            Ok(res) => {
                //println!("{:?}", res);
                if res.status() == 200 {
                    let league_response: LeagueResponse = res.json().await.unwrap();
                    //println!("{:?}", league_response);
                    if let Some(lls) = &self.state.last_league_state {
                        // Check if the client state has changed since last time checked
                        if  league_response != *lls {
                            self.state.last_league_state = Some(league_response);
                            self.state.last_request_timestamp = chrono::offset::Local::now().timestamp_millis();
                        }
                    }
                    self.state.http_client_connected = true;
                }
            },
            Err(err) => {
                println!("{} Error connecting to the league client\n{}", "[LeagueBot]", err);
                self.state.http_client_attempt_connect = false;
                self.state.http_client_connected = false;
            },
        }
    }
    /** Check if the level has changed since last checked */
    async fn check_league_client(&mut self) {
        if let Some(lls) = &self.state.last_league_state {
            if !(&self.state.is_counting) && !(&self.state.should_poll_for_level) {
                if lls.level > self.state.last_level.into() {
                    println!("[LeagueBot] Level difference: {} -> {}", self.state.last_level, lls.level);
                    // Remember to poll for level
                    self.state.should_poll_for_level = true;
                }
            }
        }
    }
}