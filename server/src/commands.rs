use common::{MenuItem, PlayerClass};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum RpgCommand {
    Join(PlayerClass),
    Rejoin,
    PlayerCommand(PlayerCommand),
    //TODO: display character details e.g. stats and equipment and abilities.
    // When would this command be available e.g. in or out of dungeon?
    // ShowCharacter,
}
/// PlayerCommand represents things players can do at any point in an adventure, if they are an adventurer
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum PlayerCommand {
    Use(MenuItem),
    Buy(MenuItem),
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Player(String);
