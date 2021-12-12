use ggez::graphics::Color;

pub enum Theme {
    LightText,
    DarkText,
    LightBackground,
    DarkBackground,
    DarkerBackground,
    Special,
    Rec,
}

impl Theme {
    pub fn get(&self) -> Color {
        // https://coolors.co/16697a-489fb5-82c0cc-ede7e3-ffa62b
        match *self {
            Theme::LightText        => Color::from_rgb(237, 231, 227),
            Theme::DarkText         => Color::from_rgb(22 , 105, 122),
            Theme::LightBackground  => Color::from_rgb(130, 192, 204),
            Theme::DarkBackground   => Color::from_rgb(22 , 105, 122),
            Theme::DarkerBackground => Color::from_rgb(35 , 31 , 32 ),
            Theme::Special          => Color::from_rgb(255, 166, 43 ),
            Theme::Rec              => Color::from_rgb(228, 87 , 46 ),
        }
    }
}