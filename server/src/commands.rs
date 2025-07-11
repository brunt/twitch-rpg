use common::PlayerClass;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum RpgCommand {
    Join(PlayerClass),
    Rejoin,
    PlayerCommand(PlayerCommand),
}
/// PlayerCommand represents things players can do at any point in an adventure, if they are an adventurer
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum PlayerCommand {
    Use(MenuItem),
    Buy(MenuItem),
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Player(String);

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct MenuItem(u8);

impl From<u8> for MenuItem {
    fn from(i: u8) -> Self {
        MenuItem(i)
    }
}
