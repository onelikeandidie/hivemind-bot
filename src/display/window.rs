use core::time;
use std::{sync::Arc, collections::HashMap};
use tokio::sync::Mutex;

use ggez::{self, ContextBuilder, event::{self, EventHandler, MouseButton}, Context, GameResult, graphics::{self, Color, Rect, DrawParam, Transform}, conf::{WindowMode, WindowSetup, NumSamples}, mint::{Vector2, Point2}, GameError, timer};

use crate::{bots::{league_bot::{LeagueBot, self}, vote_bot::{VoteBot, self}}, util::bot::Config};

use super::{symbol::{Symbol, SymbolName}, spritesheet::SpriteSheet, theme::Theme};

pub struct GameInit {
    pub league_bot: Arc<Mutex<LeagueBot>>,
    pub vote_bot: Arc<Mutex<VoteBot>>,
    pub config: Config,
}

pub struct Window {
    pub config: GameInit,
}

impl Window {
    pub async fn init_window(&self) {
        // Set the resolution
        let mode = WindowMode {
            width: 1920.,
            height: 32.,
            resizable: false,
            borderless: false,
            ..WindowMode::default()
        };
        let setup = WindowSetup {
            title: "Hivemind".to_owned(),
            samples: NumSamples::One,
            vsync: true,
            icon: "".to_owned(),
            srgb: true,
        };
        // Make a Context.
        let (mut ctx, event_loop) = ContextBuilder::new("hivemind-bot", "Hive Mind Bot")
            .window_mode(mode)
            .window_setup(setup)
            .build()
            .expect("aieee, could not create ggez context!");
        // Create an instance of your event handler.
        // Usually, you should provide it with the Context object to
        // use when setting your game up.
        let my_game = MyGame::new(&mut ctx, &self.config);
        // Run!
        event::run(ctx, event_loop, my_game);
    }
}

struct Input {
    mouse_down: bool,
    mouse_last_pos: Vector2<f32>,
    mouse_vector: Vector2<f32>,
}

impl Default for Input {
    fn default() -> Input {
        Input { 
            mouse_down: false,
            mouse_last_pos: Vector2 { x: 0.0, y: 0.0 },
            mouse_vector: Vector2 { x: 0.0, y: 0.0 }
        }
    }
}

struct BotTracker<T: Default>{
    pub was_counting: bool,
    pub vote_box: T,
    pub display: Vec<Symbol>,
}

impl<T: Default> Default for BotTracker<T> {
    fn default() -> Self {
        Self { 
            was_counting: Default::default(), 
            vote_box: Default::default(), 
            display: Default::default() 
        }
    }
}

struct MyGame {
    // Pointers
    pub league_bot: Arc<Mutex<LeagueBot>>,
    pub vote_bot: Arc<Mutex<VoteBot>>,
    // Bot Trackers
    pub l_t: BotTracker<league_bot::Votes>,
    pub v_t: BotTracker<vote_bot::Votes>,
    // Channel stats
    pub channel_name: String,
    // Display
    pub symbols: [Symbol; 62],
    pub effect_layer: [Option<Symbol>; 62],
    pub effects: Vec<u16>,
    // Timers
    pub rec_time: f32,
    // Assets
    pub textures: HashMap<String, SpriteSheet>,
    pub input: Input,
}

impl MyGame {
    pub fn new(ctx: &mut Context, game_init: &GameInit) -> MyGame {
        // Load/create resources such as images here.
        let mut textures = HashMap::new();

        let symbol_atlas = graphics::Image::new(ctx, "/symbols/atlas.png").unwrap();
        let sa_w = symbol_atlas.width();
        let sa_h = symbol_atlas.height();
        let sprite_sheet = SpriteSheet {
            tile_size: Vector2 { x: 32, y: 32 },
            tile_ratio: Vector2 { x: 32. / (sa_w as f32), y: 32. / (sa_h as f32) },
            atlas: symbol_atlas,
        };
        textures.insert("symbol_atlas".to_owned(), sprite_sheet);

        let initial_symbols = [
            Symbol {
                background_colour: Theme::DarkBackground.get(),
                colour: Theme::LightText.get(),
                ..Default::default()
            }; 
            62];

        let mut game = MyGame {
            league_bot: game_init.league_bot.clone(),
            vote_bot: game_init.vote_bot.clone(),
            channel_name: game_init.config.channel_name.clone(),
            l_t: Default::default(),
            v_t: Default::default(),
            symbols: initial_symbols,
            effect_layer: [None; 62],
            effects: Vec::new(),
            rec_time: 0.,
            textures,
            input: Default::default(),
        };
        // Return game
        game
    }

    // Drawing
    fn draw_tile(&self, ctx: &mut Context, symbol: &Symbol, vec2: Vector2<f32>) {
        // Draw Background
        let source_rect = Rect { x: vec2.x, y: vec2.y, w: 32., h: 32. };
        self.draw_rect(ctx, source_rect, symbol.background_colour);
        // Draw Symbol
        self.draw_symbol(ctx, symbol,Vector2 { x: vec2.x, y: vec2.y });
    }

    fn draw_rect(&self, ctx: &mut Context, source_rect: Rect, color: Color) {
        let rect = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(), 
            source_rect, 
            color
        ).unwrap();
        graphics::draw(ctx, &rect, DrawParam::default()).unwrap();
    }

    fn draw_symbol(&self, ctx: &mut Context, symbol: &Symbol, vec2: Vector2<f32>) {
        // Get the symbol
        let s_pos = symbol.s.atlas();
        let symbol_sheet = self.textures.get("symbol_atlas").unwrap();
        // Find the rectangle in the sprite sheet
        let w = symbol_sheet.tile_ratio.x;
        let h = symbol_sheet.tile_ratio.y;
        let s_x = s_pos.x as f32;
        let s_y = s_pos.y as f32;
        let x_start_index = s_x * w;
        let y_start_index = s_y * h;
        let x_end_index = s_x * w + w;
        let y_end_index = s_y * h + h;
        // Set the parameters
        let params = graphics::DrawParam {
            src: Rect { 
                x: x_start_index, 
                y: y_start_index, 
                w: x_end_index, 
                h: y_end_index,
            },
            color: symbol.colour,
            trans: Transform::Values {
                dest: Point2 {
                    x: (vec2.x as f32),
                    y: (vec2.y as f32)
                },
                scale: Vector2 {
                    x: 1.,
                    y: 1.
                },
                rotation: 0.,
                offset: Point2 {
                    x: 0.,
                    y: 0.
                },
            },
            ..Default::default()
        };
        graphics::draw(ctx, &symbol_sheet.atlas, params).unwrap();
    }

    // Symbols setting
    pub fn clear(&mut self) {
        self.symbols = [
            Symbol {
                background_colour: Theme::DarkBackground.get(),
                colour: Theme::LightText.get(),
                ..Default::default()
            }; 
            62];
    }

    pub fn set_symbol(&mut self, symbol: &Symbol, index: usize) {
        self.symbols[index] = symbol.clone();
    }

    pub fn set_symbols(&mut self, symbol_list: Vec<Symbol>, index: usize) {
        for i in 0..symbol_list.len() {
            let symbol = symbol_list[i];
            self.set_symbol(&symbol, index + i);
        };
    }

    pub fn set_symbol_name(&mut self, symbol: &SymbolName, index: usize) {
        self.symbols[index].s = symbol.clone();
    }

    pub fn set_char(&mut self, character: char, background: Option<Color>, index: usize) {
        let symbol = SymbolName::get(character);
        self.symbols[index] = Symbol {
            s: symbol,
            background_colour: background.unwrap_or(Color::BLACK),
            ..Default::default()
        };
    }

    pub fn set_text(&mut self, string: String, index: usize) {
        let symbol_collection = SymbolName::get_text(&string);
        for i in 0..symbol_collection.len() {
            self.set_symbol_name(&symbol_collection[i], index + i);
        };
    }

    // EVENTS
    pub fn league_bot_started_counting(&mut self) {
        self.l_t.display.clear();
        self.clear();
        println!("[Window] League bot starting counting, collecting");
    }

    pub fn league_bot_stopped_counting(&mut self) {
        self.l_t.display.clear();
        self.clear();
        println!("[Window] League bot stopped counting, clearing");
    }

    pub fn league_bot_update(&mut self, votes: league_bot::Votes) {
        self.l_t.vote_box = votes;
        let mut v_str = vec![
            " Q".to_owned() + &votes.0.to_string(), 
            " W".to_owned() + &votes.1.to_string(), 
            " E".to_owned() + &votes.2.to_string(), 
            " R".to_owned() + &votes.3.to_string(),
        ];
        let most_voted_index = votes.most_voted_index();
        // Replace first char of the most voted ability with the level icon
        if let Some(most) = most_voted_index {
            let s = v_str[most]
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    match i {
                        0 => '"',
                        _ => c
                    }
                })
                .collect::<String>();
            v_str[most] = s;
        }
        self.l_t.display.clear();
        // This is a mess
        // Basically gets collects the symbols with the highlight on the most
        // voted. I was 100% brain usage writing this but I can't remember
        // What I did.
        self.l_t.display = v_str.iter_mut()
            .enumerate()
            .flat_map(|(i, x)| {
                // Get the symbol names of the text
                let sn_collection = SymbolName::get_text(x);
                // Create the display symbol list
                let mut symbols = Vec::new();
                // Go through each and change the background of the most voted
                for j in 0..sn_collection.len() {
                    let background_colour: Color;
                    let colour: Color;
                    match most_voted_index {
                        Some(most) => {
                            if most == i {
                                background_colour = Theme::DarkerBackground.get();
                                colour = Theme::Special.get();
                            } else {
                                background_colour = Theme::LightBackground.get();
                                colour = Theme::DarkText.get();
                            }
                        },
                        _ => {
                            background_colour = Theme::LightBackground.get();
                            colour = Theme::DarkText.get();
                        }
                    }
                    // Push the symbol onto the display list
                    symbols.push(Symbol {
                        background_colour,
                        colour,
                        s: sn_collection[j],
                    });
                }
                symbols
            })
            .collect::<Vec<Symbol>>();

        self.l_t.display.insert(0, Symbol {
            s: SymbolName::LesserThen,
            colour: Theme::LightText.get(),
            background_colour: Theme::DarkerBackground.get(),
        });
        self.l_t.display.push(Symbol {
            s: SymbolName::GreaterThen,
            colour: Theme::LightText.get(),
            background_colour: Theme::DarkerBackground.get(),
        });
    }

    pub fn update_rec_symbol(&mut self, delta: time::Duration) {
        self.rec_time += delta.as_secs_f32();
        if self.rec_time > 1. {
            self.set_symbol_name(&SymbolName::RecOn, 0);
        }
        if self.rec_time > 2. {
            self.rec_time = 0.;
            self.set_symbol_name(&SymbolName::RecOff, 0);
        }
    }

    pub fn update_stream_channel(&mut self) {
        // Add the stream channel to the display
        let c_text = SymbolName::get_text(&self.channel_name.to_owned());
        let channel_symbols = c_text.iter()
            .map(|s| {
                Symbol {
                    s: *s,
                    background_colour: Theme::LightBackground.get(),
                    colour: Theme::DarkText.get(),
                }
            }).collect::<Vec<Symbol>>();
        self.set_symbols(channel_symbols, 1);
        self.symbols[0].background_colour = Theme::DarkerBackground.get();
        self.symbols[0].colour = Theme::Rec.get();
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Get delta time
        let delta = timer::delta(ctx);
        // Update data from League bot!
        {
            let arc = self.league_bot.clone();
            let league_bot = arc.try_lock();
            match league_bot {
                Ok(bot) => {
                    // Event if it just started counting
                    if self.l_t.was_counting != bot.state.is_counting {
                        if bot.state.is_counting {
                            self.league_bot_started_counting();
                        } else {
                            self.league_bot_stopped_counting();
                        }
                        self.l_t.was_counting = bot.state.is_counting;
                    }
                    // Update votes
                    if bot.state.is_counting {
                        let votes = bot.state.voting_box.clone();
                        self.league_bot_update(votes);
                    }
                },
                Err(_) => {},
            }
        }
        // Update display with the voting from the league bot
        {
            let display = self.l_t.display.clone();
            self.set_symbols(display, 19);
        }
        // Update rec symbol
        self.update_rec_symbol(delta);
        // Update title
        self.update_stream_channel();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::WHITE);
        // Draw code here...
        for i in 0..self.symbols.len() {
            self.draw_tile(ctx, &self.symbols[i], Vector2 { x: (i as f32) * 32., y: 0. });
        }
        graphics::present(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32) {}

    fn mouse_enter_or_leave(&mut self, _ctx: &mut Context, _entered: bool) {}

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) {}

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: event::KeyCode,
        _keymods: event::KeyMods,
        _repeat: bool,
    ) {
        if keycode == event::KeyCode::Escape {
            event::quit(ctx);
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: event::KeyCode, _keymods: event::KeyMods) {}

    fn text_input_event(&mut self, _ctx: &mut Context, _character: char) {}

    fn gamepad_button_down_event(&mut self, _ctx: &mut Context, _btn: event::Button, _id: event::GamepadId) {}

    fn gamepad_button_up_event(&mut self, _ctx: &mut Context, _btn: event::Button, _id: event::GamepadId) {}

    fn gamepad_axis_event(&mut self, _ctx: &mut Context, _axis: event::Axis, _value: f32, _id: event::GamepadId) {
    }

    fn focus_event(&mut self, _ctx: &mut Context, _gained: bool) {}

    fn quit_event(&mut self, _ctx: &mut Context) -> bool {
        //debug!("quit_event() callback called, quitting...");
        false
    }

    fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32) {}

    fn on_error(&mut self, _ctx: &mut Context, _origin: event::ErrorOrigin, _e: GameError) -> bool {
        true
    }
}

