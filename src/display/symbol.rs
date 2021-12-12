use ggez::{graphics::Color, mint::Vector2};

#[derive(Clone, Copy, Debug)]
pub struct Symbol {
    pub background_colour: Color,
    pub colour: Color,
    pub s: SymbolName,
}

impl Default for Symbol {
    fn default() -> Self {
        Self { background_colour: Color::BLACK, colour: Color::WHITE, s: SymbolName::Null }
    }
}

#[derive(Clone, Copy, Debug)]
#[allow(non_snake_case, non_camel_case_types)]
pub enum SymbolName {
    a, b, c, d ,e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z,
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine,
    Dot, Colon, Comma, SemiColon, 
    OpenParentisis, CloseParentisis, OpenCurly, CloseCurly, OpenSquare, CloseSquare,
    Star, ExclamationMark, QuestionMark, Caret, HashTag,
    Dollar, Percent, Ampersand, Minus, Plus, At,
    GreaterThen, LesserThen,
    Camera, Person, RecOn, RecOff, LevelUp,
    Null
}

fn new_vector2<T>(x: T, y: T) -> Vector2<T> {
    Vector2::<T> {x, y}
}

// AVERT YOUR EYEBALLS THIS IS CRINGE
impl SymbolName {
    pub fn atlas(&self) -> Vector2<u32> {
        match *self {
            SymbolName::a               => new_vector2(0 , 0),
            SymbolName::b               => new_vector2(1 , 0),
            SymbolName::c               => new_vector2(2 , 0),
            SymbolName::d               => new_vector2(3 , 0),
            SymbolName::e               => new_vector2(4 , 0),
            SymbolName::f               => new_vector2(5 , 0),
            SymbolName::g               => new_vector2(6 , 0),
            SymbolName::h               => new_vector2(7 , 0),
            SymbolName::i               => new_vector2(8 , 0),
            SymbolName::j               => new_vector2(9 , 0),
            SymbolName::k               => new_vector2(10, 0),
            SymbolName::l               => new_vector2(11, 0),
            SymbolName::m               => new_vector2(12, 0),
            SymbolName::n               => new_vector2(13, 0),
            SymbolName::o               => new_vector2(14, 0),
            SymbolName::p               => new_vector2(15, 0),
            SymbolName::q               => new_vector2(16, 0),
            SymbolName::r               => new_vector2(17, 0),
            SymbolName::s               => new_vector2(18, 0),
            SymbolName::t               => new_vector2(19, 0),
            SymbolName::u               => new_vector2(20, 0),
            SymbolName::v               => new_vector2(21, 0),
            SymbolName::w               => new_vector2(22, 0),
            SymbolName::x               => new_vector2(23, 0),
            SymbolName::y               => new_vector2(24, 0),
            SymbolName::z               => new_vector2(25, 0),
            SymbolName::A               => new_vector2(0 , 1),
            SymbolName::B               => new_vector2(1 , 1),
            SymbolName::C               => new_vector2(2 , 1),
            SymbolName::D               => new_vector2(3 , 1),
            SymbolName::E               => new_vector2(4 , 1),
            SymbolName::F               => new_vector2(5 , 1),
            SymbolName::G               => new_vector2(6 , 1),
            SymbolName::H               => new_vector2(7 , 1),
            SymbolName::I               => new_vector2(8 , 1),
            SymbolName::J               => new_vector2(9 , 1),
            SymbolName::K               => new_vector2(10, 1),
            SymbolName::L               => new_vector2(11, 1),
            SymbolName::M               => new_vector2(12, 1),
            SymbolName::N               => new_vector2(13, 1),
            SymbolName::O               => new_vector2(14, 1),
            SymbolName::P               => new_vector2(15, 1),
            SymbolName::Q               => new_vector2(16, 1),
            SymbolName::R               => new_vector2(17, 1),
            SymbolName::S               => new_vector2(18, 1),
            SymbolName::T               => new_vector2(19, 1),
            SymbolName::U               => new_vector2(20, 1),
            SymbolName::V               => new_vector2(21, 1),
            SymbolName::W               => new_vector2(22, 1),
            SymbolName::X               => new_vector2(23, 1),
            SymbolName::Y               => new_vector2(24, 1),
            SymbolName::Z               => new_vector2(25, 1),
            SymbolName::Zero            => new_vector2(0 , 2),
            SymbolName::One             => new_vector2(1 , 2),
            SymbolName::Two             => new_vector2(2 , 2),
            SymbolName::Three           => new_vector2(3 , 2),
            SymbolName::Four            => new_vector2(4 , 2),
            SymbolName::Five            => new_vector2(5 , 2),
            SymbolName::Six             => new_vector2(6 , 2),
            SymbolName::Seven           => new_vector2(7 , 2),
            SymbolName::Eight           => new_vector2(8 , 2),
            SymbolName::Nine            => new_vector2(9 , 2),
            SymbolName::Dot             => new_vector2(10, 2),
            SymbolName::Colon           => new_vector2(11, 2),
            SymbolName::Comma           => new_vector2(12, 2),
            SymbolName::SemiColon       => new_vector2(13, 2),
            SymbolName::OpenParentisis  => new_vector2(14, 2),
            SymbolName::CloseParentisis => new_vector2(20, 2),
            SymbolName::OpenCurly       => new_vector2(23, 2),
            SymbolName::CloseCurly      => new_vector2(18, 2),
            SymbolName::OpenSquare      => new_vector2(3 , 3),
            SymbolName::CloseSquare     => new_vector2(4 , 3),
            SymbolName::Star            => new_vector2(15, 2),
            SymbolName::ExclamationMark => new_vector2(16, 2),
            SymbolName::QuestionMark    => new_vector2(17, 2),
            SymbolName::Caret           => new_vector2(19, 2),
            SymbolName::HashTag         => new_vector2(21, 2),
            SymbolName::Dollar          => new_vector2(22, 2),
            SymbolName::Percent         => new_vector2(24, 2),
            SymbolName::Ampersand       => new_vector2(25, 2),
            SymbolName::Minus           => new_vector2(0 , 3),
            SymbolName::Plus            => new_vector2(1 , 3),
            SymbolName::At              => new_vector2(2 , 3),
            SymbolName::GreaterThen     => new_vector2(6 , 3),
            SymbolName::LesserThen      => new_vector2(5 , 3),
            SymbolName::Camera          => new_vector2(0 , 4),
            SymbolName::Person          => new_vector2(1 , 4),
            SymbolName::RecOn           => new_vector2(2 , 4),
            SymbolName::RecOff          => new_vector2(3 , 4),
            SymbolName::LevelUp         => new_vector2(4 , 4),
            SymbolName::Null            => new_vector2(25, 7),
        }
    }

    pub fn get(character: char) -> SymbolName {
        match character {
            'a' => SymbolName::a,
            'b' => SymbolName::b,
            'c' => SymbolName::c,
            'd' => SymbolName::d,
            'e' => SymbolName::e,
            'f' => SymbolName::f,
            'g' => SymbolName::g,
            'h' => SymbolName::h,
            'i' => SymbolName::i,
            'j' => SymbolName::j,
            'k' => SymbolName::k,
            'l' => SymbolName::l,
            'm' => SymbolName::m,
            'n' => SymbolName::n,
            'o' => SymbolName::o,
            'p' => SymbolName::p,
            'q' => SymbolName::q,
            'r' => SymbolName::r,
            's' => SymbolName::s,
            't' => SymbolName::t,
            'u' => SymbolName::u,
            'v' => SymbolName::v,
            'w' => SymbolName::w,
            'x' => SymbolName::x,
            'y' => SymbolName::y,
            'z' => SymbolName::z,
            'A' => SymbolName::A,
            'B' => SymbolName::B,
            'C' => SymbolName::C,
            'D' => SymbolName::D,
            'E' => SymbolName::E,
            'F' => SymbolName::F,
            'G' => SymbolName::G,
            'H' => SymbolName::H,
            'I' => SymbolName::I,
            'J' => SymbolName::J,
            'K' => SymbolName::K,
            'L' => SymbolName::L,
            'M' => SymbolName::M,
            'N' => SymbolName::N,
            'O' => SymbolName::O,
            'P' => SymbolName::P,
            'Q' => SymbolName::Q,
            'R' => SymbolName::R,
            'S' => SymbolName::S,
            'T' => SymbolName::T,
            'U' => SymbolName::U,
            'V' => SymbolName::V,
            'W' => SymbolName::W,
            'X' => SymbolName::X,
            'Y' => SymbolName::Y,
            'Z' => SymbolName::Z,
            '0' => SymbolName::Zero,
            '1' => SymbolName::One,
            '2' => SymbolName::Two,
            '3' => SymbolName::Three,
            '4' => SymbolName::Four,
            '5' => SymbolName::Five,
            '6' => SymbolName::Six,
            '7' => SymbolName::Seven,
            '8' => SymbolName::Eight,
            '9' => SymbolName::Nine,
            '.' => SymbolName::Dot,
            ':' => SymbolName::Colon,
            ',' => SymbolName::Comma,
            ';' => SymbolName::SemiColon,
            '(' => SymbolName::OpenParentisis,
            ')' => SymbolName::CloseParentisis,
            '{' => SymbolName::OpenCurly,
            '}' => SymbolName::CloseCurly,
            '[' => SymbolName::OpenSquare,
            ']' => SymbolName::CloseSquare,
            '*' => SymbolName::Star,
            '!' => SymbolName::ExclamationMark,
            '?' => SymbolName::QuestionMark,
            '^' => SymbolName::Caret,
            '#' => SymbolName::HashTag,
            '$' => SymbolName::Dollar,
            '%' => SymbolName::Percent,
            '&' => SymbolName::Ampersand,
            '-' => SymbolName::Minus,
            '+' => SymbolName::Plus,
            '@' => SymbolName::At,
            '>' => SymbolName::GreaterThen,
            '<' => SymbolName::LesserThen,
            '"' => SymbolName::LevelUp,
            _ => SymbolName::Null
        }
    }

    pub fn get_text(text: &str) -> Vec<SymbolName> {
        let mut symbol_collection = Vec::new();
        let mut chars = text.chars();
        let mut character = chars.next();
        while character.is_some() {
            match character {
                Some(c) => {
                    let symbol = SymbolName::get(c);
                    symbol_collection.push(symbol);
                    character = chars.next();
                },
                None => {

                },
            }
        }
        symbol_collection
    }
}