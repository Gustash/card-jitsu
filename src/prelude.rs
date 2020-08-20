use crate::models::{Card, Color, Element};
use crate::result::Error;
use rand::Rng;
use std::fmt;

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::RED => write!(f, "Red"),
            Color::GREEN => write!(f, "Green"),
            Color::BLUE => write!(f, "Blue"),
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::FIRE => write!(f, "Fire"),
            Element::SNOW => write!(f, "Snow"),
            Element::WATER => write!(f, "Water"),
        }
    }
}

impl Card {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        Card {
            color: match rng.gen_range(0, 3) {
                0 => Color::RED,
                1 => Color::GREEN,
                2 => Color::BLUE,
                _ => Color::RED,
            },
            element: match rng.gen_range(0, 3) {
                0 => Element::FIRE,
                1 => Element::SNOW,
                2 => Element::WATER,
                _ => Element::FIRE,
            },
            value: rng.gen_range(1, 11),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ExistingActiveChallenge(user_id) => {
                write!(f, "User {} has an active challenge", user_id)
            }
            Error::HandleCommand(error) => write!(f, "{}", error),
            Error::Serenity(error) => write!(f, "{}", error),
            Error::SQLX(error) => write!(f, "{}", error),
        }
    }
}
