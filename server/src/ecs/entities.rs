// In ecs/entities.rs
use crate::ecs::components::{
    CharacterClass, Equipment, Experience, Health, HealthComponent, MovementAI, MovementAIKind,
    Resource, Stats,
};
use common::PlayerClass;
use specs::{Entities, Entity, LazyUpdate, ReadExpect};

// Helper functions to determine starting attributes based on class
fn get_starting_hp(class: &PlayerClass) -> u32 {
    match class {
        PlayerClass::Fighter | PlayerClass::Paladin => 20,
        PlayerClass::Cleric | PlayerClass::Druid => 16,
        PlayerClass::Monk | PlayerClass::Ranger => 14,
        PlayerClass::Rogue => 12,
        PlayerClass::Warlock | PlayerClass::Wizard => 10,
    }
}

//TODO: do I even want mana
fn get_starting_mana(class: &PlayerClass) -> u32 {
    match class {
        PlayerClass::Wizard | PlayerClass::Warlock => 30,
        PlayerClass::Cleric | PlayerClass::Druid => 25,
        PlayerClass::Paladin => 15,
        PlayerClass::Ranger => 10,
        PlayerClass::Fighter | PlayerClass::Monk | PlayerClass::Rogue => 5,
    }
}

fn get_starting_stats(class: &PlayerClass) -> Stats {
    match class {
        PlayerClass::Fighter => Stats {
            strength: 16,
            agility: 12,
            intelligence: 8,
            vitality: 14,
        },
        PlayerClass::Wizard => Stats {
            strength: 6,
            agility: 10,
            intelligence: 18,
            vitality: 8,
        },
        PlayerClass::Rogue => Stats {
            strength: 10,
            agility: 18,
            intelligence: 12,
            vitality: 10,
        },
        // Add cases for other classes
        _ => Stats {
            strength: 10,
            agility: 10,
            intelligence: 10,
            vitality: 10,
        },
    }
}
