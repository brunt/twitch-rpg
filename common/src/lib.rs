use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameSnapShot {
    #[serde(rename = "a")]
    pub party: Vec<PlayerSnapshot>,
    #[serde(rename = "b")]
    pub camera_position: Option<tatami_dungeon::Position>,
    #[serde(rename = "c")]
    pub floor_positions: Option<Vec<EntityPosition>>, //TODO: use component position type?
    #[serde(rename = "d")]
    pub floor: Option<Vec<Vec<u8>>>,
    #[serde(rename = "e")]
    pub shop_items: Option<HashMap<MenuItem, ShopItem>>,
    #[serde(rename = "f")]
    pub ready_timer: Option<SerializedCountdownTimer>,
    #[serde(rename = "g")]
    pub difficulty: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityPosition {
    #[serde(rename = "a")]
    pub entity_type: String,
    #[serde(rename = "b")]
    pub position: tatami_dungeon::Position,
    #[serde(rename = "c")]
    pub level: u32,
    #[serde(rename = "d")]
    pub target_position: Option<tatami_dungeon::Position>,
    #[serde(rename = "e")]
    pub health: Option<Health>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerSnapshot {
    #[serde(rename = "a")]
    pub name: String,
    #[serde(rename = "b")]
    pub class: PlayerClass,
    #[serde(rename = "c")]
    pub health: Health,
    #[serde(rename = "d")]
    pub level: u32,
    #[serde(rename = "e")]
    pub gold: u32,
    #[serde(rename = "f")]
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

#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct AttackModifiers {
    pub damage_bonus: i32,
    pub hit_rating_bonus: i32,
    pub range_bonus: i32,
    pub cooldown_reduction_ms: i32, // negative for faster attacking
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DefenseModifiers {
    pub damage_reduction: i32,
    pub evasion_rating: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OtherModifiers {
    pub movement_speed_increase: u32
    // others as necessary
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemStats {
    //TODO: too many Option<>s?
    /// direct modifiers to attack components
    pub attack_modifiers: Option<AttackModifiers>,
    /// direct modifiers to defense components
    pub defense_modifiers: Option<DefenseModifiers>,
    
    /// other modifiers
    pub other_modifiers: Option<OtherModifiers>,
    
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