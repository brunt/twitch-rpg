pub(crate) use crate::ecs::components::movement::Position;
pub(crate) use common::PlayerClass;
use common::PlayerStats;
use specs::prelude::*;
use specs_derive::Component;

pub mod class;
pub mod combat;
pub mod effect;
pub mod form;
pub mod inventory;
pub mod movement;
pub mod spells;

// Corpse component for defeated entities
#[derive(Debug, Component)]
#[storage(NullStorage)]
pub struct Corpse;

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
        Self(20)
    }
}
