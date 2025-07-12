use std::fmt::Display;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use tatami_dungeon::Floor;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameSnapShot {
    pub party: Vec<PlayerSnapshot>,
    pub floor_positions: Option<Vec<String>>,
    pub floor: Option<Floor>,
    pub shop_items: Option<Vec<ShopItem>>,
    pub ready_timer: Option<SerializedCountdownTimer>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerSnapshot {
    pub name: String,
    pub class: Option<PlayerClass>, // Class -> PlayerClass
    pub health: Health,
    pub level: u32,
    pub gold: u32,
    pub sprite_name: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PlayerClass {
    Fighter,
    Druid,
    Wizard,
    Ranger,
    Rogue,
    Cleric,
    Paladin,
    Warlock,
    Monk,
    Sorcerer,
}

impl PlayerClass {}

impl Display for PlayerClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Fighter => write!(f, "Fighter"),
            Self::Druid => write!(f, "Druid"),
            Self::Wizard => write!(f, "Wizard"),
            Self::Ranger => write!(f, "Ranger"),
            Self::Rogue => write!(f, "Rogue"),
            Self::Cleric => write!(f, "Cleric"),
            Self::Paladin => write!(f, "Paladin"),
            Self::Warlock => write!(f, "Warlock"),
            Self::Monk => write!(f, "Monk"),
            Self::Sorcerer => write!(f, "Sorcerer"),
        }
    }
}

impl FromStr for PlayerClass {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fighter" => Ok(Self::Fighter),
            "druid" => Ok(Self::Druid),
            "wizard" => Ok(Self::Wizard),
            "ranger" => Ok(Self::Ranger),
            "rogue" => Ok(Self::Rogue),
            "cleric" => Ok(Self::Cleric),
            "paladin" => Ok(Self::Paladin),
            "warlock" => Ok(Self::Warlock),
            "monk" => Ok(Self::Monk),
            "sorcerer" => Ok(Self::Sorcerer),
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid player class"))
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Health {
    Alive { hp: u32, max_hp: u32 },
    Dead,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShopItem {
    pub sprite: String,
    pub name: String,
    pub price: u32,
    pub description: String,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct SerializedCountdownTimer {
    pub remaining: u64,
}