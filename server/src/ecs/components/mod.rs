pub(crate) use crate::ecs::components::movement::Position;
use common::PlayerStats;
pub(crate) use common::{Health, PlayerClass};
use specs::prelude::*;
use specs_derive::Component;
use std::fmt::Display;

pub mod class;
pub mod combat;
pub mod effect;
pub mod form;
pub mod inventory;
pub mod movement;

// Corpse component for defeated entities
#[derive(Debug, Component)]
#[storage(NullStorage)]
pub struct Corpse;

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

impl Default for Experience {
    fn default() -> Self {
        Self {
            current: 0,
            next_level: 50,
        }
    }
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

#[derive(Debug, Component, Clone, Default)]
#[storage(NullStorage)]
pub struct Enemy;

// TODO: delete?
#[derive(Debug, Component, Clone, Default)]
#[storage(NullStorage)]
pub struct DungeonItem;

/// for chests and doors-- retain visibility
#[derive(Debug, Component, Clone, Default)]
#[storage(NullStorage)]
pub struct Opened;

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

// Projectile effects
#[derive(Debug, Clone)]
pub enum ProjectileEffect {
    Damage(u32),
    Slow(f32, std::time::Duration),
    Stun(std::time::Duration),
    AreaOfEffect { radius: u32, effect_type: String },
}

// Experience reward component
#[derive(Debug, Component)]
pub struct ExperienceReward {
    pub base_amount: u32,
    pub level_multiplier: f32, // Multiplier based on entity level
}

/// Stats component for RPG attributes
#[derive(Debug, Component, Clone)]
pub struct Stats {
    pub strength: u32,
    pub agility: u32,
    pub intelligence: u32,
}

//TODO: stats for enemies
// TODO: stats based on level
impl Stats {
    /// starting stats sum to 30?
    pub(crate) fn new(class: &PlayerClass) -> Self {
        match class {
            PlayerClass::Fighter => Stats {
                strength: 12,
                agility: 10,
                intelligence: 8,
            },
            PlayerClass::Paladin => Stats {
                strength: 12,
                agility: 8,
                intelligence: 10,
            },
            PlayerClass::Wizard | PlayerClass::Sorcerer | PlayerClass::Warlock => Stats {
                strength: 6,
                agility: 10,
                intelligence: 14,
            },
            PlayerClass::Rogue | PlayerClass::Ranger => Stats {
                strength: 10,
                agility: 14,
                intelligence: 6,
            },
            PlayerClass::Druid => Stats {
                strength: 10,
                agility: 8,
                intelligence: 12,
            },
            PlayerClass::Cleric => Stats {
                strength: 9,
                agility: 9,
                intelligence: 12,
            },
        }
    }
}

impl From<&Stats> for PlayerStats {
    fn from(stats: &Stats) -> Self {
        PlayerStats {
            strength: stats.strength,
            agility: stats.agility,
            intelligence: stats.intelligence,
        }
    }
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

impl Name {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}

impl Default for Name {
    fn default() -> Self {
        Self(String::from("Enemy"))
    }
}

#[derive(Debug, Component, Clone)]
pub struct Money(pub u32);

impl Money {
    pub fn new(value: u32) -> Self {
        Self(value)
    }
}

impl Default for Money {
    fn default() -> Self {
        Self(15)
    }
}
