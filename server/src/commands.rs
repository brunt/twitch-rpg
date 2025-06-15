use std::fmt::Display;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::player_class::PlayerClass;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum RpgCommand {
    New(PlayerClass),
    Load,
    PlayerCommand(PlayerCommand),
}
/// PlayerCommand represents things players can do at any point in an adventure
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum PlayerCommand {
    Use(Player, MenuItem),
    Buy(Player, MenuItem),
}


#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Player(String);

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct MenuItem(u8);

#[derive(Clone)]
pub enum Item {
    Consumable(Consumable),
    Equipment(String),
}
impl FromStr for Item {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // "" => {
            //     Item::Equipment(s.to_string())
            // },
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
pub enum Consumable {
    Potion(String),
    Scroll(String),
}
