pub(crate) use common::{Health, PlayerClass};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

pub mod class;

// Entity's coordinates in the world
#[derive(Debug, Component, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Component, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct MovementSpeed(pub i32);

impl PartialEq<i32> for MovementSpeed {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}
impl PartialEq<MovementSpeed> for i32 {
    fn eq(&self, other: &MovementSpeed) -> bool {
        *self == other.0
    }
}
impl PartialOrd<i32> for MovementSpeed {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<MovementSpeed> for i32 {
    fn partial_cmp(&self, other: &MovementSpeed) -> Option<Ordering> {
        self.partial_cmp(&other.0)
    }
}

#[derive(Debug, Component)]
pub struct TargetPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Component)]
pub struct HealthComponent(pub Health);

impl Default for HealthComponent {
    fn default() -> Self {
        Self(Health::Alive { hp: 1, max_hp: 1 })
    }
}

impl HealthComponent {
    pub fn is_alive(&self) -> bool {
        match self.0 {
            Health::Alive { .. } => true,
            _ => false,
        }
    }

    pub fn new_from_class(class: &PlayerClass) -> Self {
        Self(match class {
            PlayerClass::Fighter | PlayerClass::Paladin | PlayerClass::Ranger => {
                Health::Alive { hp: 10, max_hp: 10 }
            }
            PlayerClass::Rogue
            | PlayerClass::Cleric
            | PlayerClass::Druid
            // | PlayerClass::Monk
            | PlayerClass::Warlock => Health::Alive { hp: 8, max_hp: 8 },
            PlayerClass::Wizard | PlayerClass::Sorcerer => Health::Alive { hp: 6, max_hp: 6 },
        })
    }
}

// Corpse component for defeated entities
#[derive(Debug, Component)]
pub struct Corpse {
    //pub time_of_death: std::time::Instant,
}

// Equipment types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeaponType {
    Sword,
    Axe,
    Bow,
    Staff,
    Dagger,
    Wand,
    Mace,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArmorType {
    Head,
    Chest,
    Jewelry,
    Hands,
    Feet,
    Legs,
}

// Equipment component
#[derive(Debug, Component, Clone)]
pub struct Equipment {
    pub weapon: Option<Weapon>,
    pub armor: Option<Armor>,
}

// Weapon component
#[derive(Debug, Clone)]
pub struct Weapon {
    pub name: String,
    pub weapon_type: WeaponType,
    pub damage: u32,
    pub attributes: Vec<ItemAttribute>,
}

// Armor component
#[derive(Debug, Clone)]
pub struct Armor {
    pub name: String,
    pub armor_type: ArmorType,
    pub defense: u32,
    pub attributes: Vec<ItemAttribute>,
}

// Item attributes for equipment
#[derive(Debug, Clone)]
pub enum ItemAttribute {
    StrengthBonus(u32),
    AgilityBonus(u32),
    IntelligenceBonus(u32),
    VitalityBonus(u32),
    FireDamage(u32),
    IceDamage(u32),
    LightningDamage(u32),
}

// Experience and level
#[derive(Debug, Component, Clone)]
pub struct Experience {
    pub current: u32,
    pub next_level: u32,
}

// Level component
#[derive(Debug, Component, Clone)]
pub struct Level(pub u32);

impl Default for Level {
    fn default() -> Self {
        Self(1)
    }
}

#[derive(Debug, Component, Clone, Default)]
#[storage(NullStorage)]
pub struct Player;

// Immobile component for entities that can't move
#[derive(Component, Debug)]
#[storage(NullStorage)]
pub struct Immobile;

// Light radius component for vision
#[derive(Debug, Component, Clone)]
pub struct LightRadius {
    pub radius: u32, // How far the entity can see in tiles
}

#[derive(Debug, Component, Clone)]
pub struct Spell {
    pub name: String,
    pub mana_cost: u32,
    pub cooldown: std::time::Duration,
    pub last_cast: Option<std::time::Instant>,
    pub spell_type: SpellType,
}

#[derive(Debug, Clone)]
pub enum SpellType {
    Damage {
        base_damage: u32,
        range: u32, // How far it can be cast from the caster
    },
    Heal {
        heal_amount: u32,
        range: u32,
    },
    // Area of Effect spell variant
    AreaOfEffect {
        effect: AoEEffect, // Specific effect (e.g., damage, heal, debuff)
        radius: u32,       // Radius of the effect area in tiles
        range: u32,        // How far from caster it can be targeted
    },
    // ...other spell types
}

// Buff effects
#[derive(Debug, Clone)]
pub enum BuffEffect {
    TemporaryDamage(u32),
    TemporaryDefense(u32),
    TemporarySpeed(i32),
    Regeneration(u32),
}

#[derive(Debug, Clone)]
pub enum AoEEffect {
    Damage(u32),        // Flat damage to all affected targets
    Heal(u32),          // Flat heal to all affected targets
    Buff(BuffEffect),   // Buff all affected targets
    Debuff(BuffEffect), // Debuff all affected targets
}

// Projectile component
#[derive(Debug, Component)]
pub struct Projectile {
    pub damage: u32,
    pub source_entity: Entity,
    pub target_position: Position,
    pub speed: f32,
    pub effects: Vec<ProjectileEffect>,
}

// Projectile effects
#[derive(Debug, Clone)]
pub enum ProjectileEffect {
    Damage(u32),
    Slow(f32, std::time::Duration),
    Stun(std::time::Duration),
    AreaOfEffect { radius: u32, effect_type: String },
}

// Loot drop component
#[derive(Debug, Component)]
pub struct LootTable {
    pub drops: Vec<LootDrop>,
}

// Individual loot drops with probabilities
#[derive(Debug, Clone)]
pub struct LootDrop {
    pub item_type: LootType,
    pub probability: f32,           // 0.0 to 1.0
    pub quantity_range: (u32, u32), // Min and max quantity
}

// Types of loot
#[derive(Debug, Clone)]
pub enum LootType {
    Gold,
    Weapon(WeaponType),
    Armor(ArmorType),
    Potion,
    Scroll,
    Gem,
    QuestItem(String),
}

// Experience reward component
#[derive(Debug, Component)]
pub struct ExperienceReward {
    pub base_amount: u32,
    pub level_multiplier: f32, // Multiplier based on entity level
}

// Stats component for RPG attributes
#[derive(Debug, Component, Clone)]
pub struct Stats {
    pub strength: u32,
    pub agility: u32,
    pub intelligence: u32,
    pub vitality: u32,
}

// Active effects component (buffs/debuffs)
#[derive(Debug, Component)]
pub struct ActiveEffects {
    pub effects: Vec<TimedEffect>,
}

// Timed effect for buffs/debuffs
#[derive(Debug, Clone)]
pub struct TimedEffect {
    pub effect_type: BuffEffect,
    pub start_time: std::time::Instant,
    pub duration: std::time::Duration,
}

// Resource component for mana, stamina, etc.
#[derive(Debug, Component, Clone)]
pub struct Resource {
    pub mana: u32,
    pub max_mana: u32,
    pub stamina: u32,
    pub max_stamina: u32,
}

#[derive(Debug, Component, Clone)]
pub struct MovementAI {
    pub kind: MovementAIKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MovementAIKind {
    /// Non-player entity: wander randomly from point to point
    WanderRandomly,
    /// Player: pathfind toward unexplored rooms, seeking the next floor entrance
    ExploreDungeon,
    /// Entity follows an explicit path (e.g., chasing, patrolling), path to be computed elsewhere
    PathfindTo(Vec<(i32, i32)>),
    /// Follow Leader
    Follow,
}

#[derive(Debug, Component, Clone)]
pub struct Name(pub String);

#[derive(Debug, Component, Clone)]
pub struct Money(pub u32);

impl Default for Money {
    fn default() -> Self {
        Self(0)
    }
}
