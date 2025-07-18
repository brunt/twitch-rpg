use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameSnapShot {
    pub party: Vec<PlayerSnapshot>,
    pub party_position: Option<tatami_dungeon::Position>,
    pub floor_positions: Option<Vec<EntityPosition>>, //TODO: use component position type?
    pub floor: Option<Vec<Vec<u8>>>,
    pub shop_items: Option<HashMap<MenuItem, ShopItem>>,
    pub ready_timer: Option<SerializedCountdownTimer>,
    pub difficulty: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityPosition {
    pub class: String,
    pub position: tatami_dungeon::Position,
    pub level: u32,
    /// maybe this helps smooth animations
    pub target_position: Option<tatami_dungeon::Position>,
}

// pub struct Position {
//     pub x: u32,
//     pub y: u32,
// }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerSnapshot {
    pub name: String,
    pub class: PlayerClass,
    pub health: Health,
    pub level: u32,
    pub gold: u32,
    pub form: Form,
    // pub buffs: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Form {
    Normal,
    Polymorphed(String),
    Invisible,
    Scaled(f64), // larger or smaller
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
    // Monk,
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
            // Self::Monk => write!(f, "Monk"),
            Self::Sorcerer => write!(f, "Sorcerer"),
        }
    }
}

impl FromStr for PlayerClass {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Fighter" => Ok(Self::Fighter),
            "Druid" => Ok(Self::Druid),
            "Wizard" => Ok(Self::Wizard),
            "Ranger" => Ok(Self::Ranger),
            "Rogue" => Ok(Self::Rogue),
            "Cleric" => Ok(Self::Cleric),
            "Paladin" => Ok(Self::Paladin),
            "Warlock" => Ok(Self::Warlock),
            // "monk" => Ok(Self::Monk),
            "Sorcerer" => Ok(Self::Sorcerer),
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid player class"))
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Health {
    Alive { hp: u32, max_hp: u32 },
    Dead,
}

impl Display for Health {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Alive { hp, max_hp } => {
                let total_hearts = 6;
                let filled_hearts = (((hp * total_hearts) + (max_hp - 1)) / max_hp).min(total_hearts);
                let empty_hearts = total_hearts - filled_hearts;

                let hearts: String = "❤️".repeat(filled_hearts as usize) + &"🖤".repeat(empty_hearts as usize);
                write!(f, "{}", hearts)
            }
            Self::Dead => write!(f, "☠️ Dead"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShopItem {
    pub sprite: String,
    pub name: String,
    pub quality: ItemQuality,
    pub equip_slot: EquipmentSlot,
    pub price: u32,
    pub stats: ItemStats,
    pub description: String,
}

impl ShopItem {
    pub fn to_equipped_item(&self) -> EquippedItem {
        EquippedItem {
            name: self.name.clone(),
            quality: self.quality.clone(),
            stats: self.stats.clone(),
            description: self.description.clone(),
            slot: self.equip_slot.clone(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ItemQuality {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct SerializedCountdownTimer {
    pub remaining: u64,
}

#[derive(Hash, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct MenuItem(pub usize);

impl From<usize> for MenuItem {
    fn from(i: usize) -> Self {
        MenuItem(i)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum EquipmentSlot {
    MainHand,
    // OffHand, //TODO: too lazy to differentiate 1h/2h weapons rn
    Head,
    Body,
    Legs,
    Feet,
    Finger,
    Neck,
    UtilitySlot
}

impl Display for EquipmentSlot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MainHand => write!(f, "Main Hand"),
            Self::Head => write!(f, "Head"),
            Self::Body => write!(f, "Body"),
            Self::Legs => write!(f, "Legs"),
            Self::Feet => write!(f, "Feet"),
            Self::Finger => write!(f, "Finger"),
            Self::Neck => write!(f, "Neck"),
            Self::UtilitySlot => write!(f, "Utility Slot"), 
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EquippedItem {
    pub name: String,
    pub quality: ItemQuality,
    pub slot: EquipmentSlot,
    pub description: String,
    pub stats: ItemStats,
    //TODO: how to grant abilities?
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemStats {
    /// physical damage, health
    pub strength: Option<u32>,
    /// spell damage, duration, radius
    pub intelligence: Option<u32>,
    /// hit rating, evasion rating
    pub dexterity: Option<u32>,
}

// TODO: only show these in the Display command
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerStats {
    pub strength: u32,
    pub intelligence: u32,
    pub dexterity: u32,
}