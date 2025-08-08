use assets_manager::asset::Asset;
use assets_manager::{BoxedError, FileAsset};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::time::Duration;
use tatami_dungeon::Position;

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
    #[serde(rename = "h")]
    pub projectiles: Option<Vec<Projectile>>,
    #[serde(rename = "i")]
    pub loot: Option<u32>,
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
    pub health: Option<Health>,
    #[serde(rename = "f")]
    pub form: Form,
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
    #[serde(rename = "g")]
    pub stats: PlayerStats,
    #[serde(rename = "h")]
    pub show: bool,
    #[serde(rename = "i")]
    pub equipped_items: HashMap<EquipmentSlot, Item>,
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "invalid player class",
            )),
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
                let filled_hearts =
                    (((hp * total_hearts) + (max_hp - 1)) / max_hp).min(total_hearts);
                let empty_hearts = total_hearts - filled_hearts;

                let hearts: String =
                    "❤️".repeat(filled_hearts as usize) + &"🖤".repeat(empty_hearts as usize);
                write!(f, "{}", hearts)
            }
            Self::Dead => write!(f, "☠️ Dead"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShopItem {
    pub id: usize,
    pub name: String,
    pub quality: ItemQuality,
    pub equip_slot: EquipmentSlot,
    pub price: u32,
    pub stats: Option<ItemStats>,
    pub consumable: bool,
    pub effects: Option<Vec<(Effect, Option<f64>)>>,
    pub description: String,
}

impl ShopItem {
    pub fn to_item(&self) -> Item {
        Item {
            item_id: self.id,
            name: self.name.clone(),
            quality: self.quality.clone(),
            stats: self.stats.clone(),
            equip_slot: self.equip_slot.clone(),
            consumable: self.consumable,
            effects: self.effects.clone(),
            description: self.description.clone(),
        }
    }
}

impl From<&Item> for ShopItem {
    fn from(item: &Item) -> Self {
        item.to_shop_item()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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
    UtilitySlot,
}

impl EquipmentSlot {
    pub fn slot_order(&self) -> usize {
        match self {
            Self::MainHand => 0,
            Self::Head => 1,
            Self::Body => 2,
            Self::Legs => 3,
            Self::Feet => 4,
            Self::Finger => 5,
            Self::Neck => 6,
            Self::UtilitySlot => 7,
        }
    }
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
pub struct Item {
    /// item_id retrieves a sprite from the sprite hashmap, also facilitates database operations
    pub item_id: usize,
    pub name: String,
    pub quality: ItemQuality,
    pub equip_slot: EquipmentSlot,
    pub stats: Option<ItemStats>,
    pub consumable: bool,
    pub effects: Option<Vec<(Effect, Option<f64>)>>,
    pub description: String,
}

impl FileAsset for Item {
    const EXTENSION: &'static str = "ron";

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Result<Self, BoxedError> {
        assets_manager::asset::load_ron(&bytes)
    }
}

impl Item {
    pub fn to_shop_item(&self) -> ShopItem {
        let base_stats_price = self.stats.as_ref().map_or(0, |s| {
            2 * (s.strength.unwrap_or(0)
                + s.agility.unwrap_or(0)
                + s.intelligence.unwrap_or(0)
                + if s.attack_modifiers.is_some() { 5 } else { 0 }
                + if s.defense_modifiers.is_some() { 5 } else { 0 }
                + if s.other_modifiers.is_some() { 5 } else { 0 })
        });

        let effects_price: u32 = self.effects.as_ref().map_or(0, |effects| {
            effects.iter().fold(0, |acc, (effect, duration)| {
                acc + match effect {
                    Effect::Heal(amount) => *amount * 3,
                    Effect::Transform(Form::Scaled(_)) => 10,
                    Effect::Transform(Form::Invisible) => 20,
                    _ => 20,
                } * duration.map_or(1, |d| (d / 60.0) as u32)
            })
        });

        let price = base_stats_price + effects_price;

        ShopItem {
            id: self.item_id,
            name: self.name.clone(),
            quality: self.quality.clone(),
            equip_slot: self.equip_slot.clone(),
            price,
            stats: self.stats.clone(),
            consumable: self.consumable,
            effects: self.effects.clone(),
            description: self.description.clone(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub movement_speed_increase: u32, // others as necessary
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
    pub agility: Option<u32>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StatChange {
    pub strength: Option<i32>,
    pub intelligence: Option<i32>,
    pub agility: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PlayerStats {
    #[serde(rename = "a")]
    pub strength: u32,
    #[serde(rename = "b")]
    pub intelligence: u32,
    #[serde(rename = "c")]
    pub agility: u32,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Projectile {
    #[serde(rename = "a")]
    pub position: Position,
    #[serde(rename = "b")]
    pub target_position: Position,
    #[serde(rename = "c")]
    pub damage: u32, // for scrolling combat text maybe
    #[serde(rename = "d")]
    pub damage_type: DamageType,
}

/// this is used for rendering so basic color names are fine
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DamageType {
    Physical,
    Red,
    Blue,
    Green,
    Yellow,
    Orange,
    Purple,
    Pink,
    Black,
}

impl Display for DamageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Physical => write!(f, "Physical"),
            Self::Red => write!(f, "Red"),
            Self::Blue => write!(f, "Blue"),
            Self::Green => write!(f, "Green"),
            Self::Yellow => write!(f, "Yellow"),
            Self::Orange => write!(f, "Orange"),
            Self::Purple => write!(f, "Purple"),
            Self::Pink => write!(f, "Pink"),
            Self::Black => write!(f, "Black"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Effect {
    Heal(u32),
    Revive,
    Transform(Form),
    StatChange(StatChange),
    // GainAbility
}
