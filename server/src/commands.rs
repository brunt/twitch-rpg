use common::{MenuItem, PlayerClass};
use serde::{Deserialize, Serialize};

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
    Show,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Player(String);
