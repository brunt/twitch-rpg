// In ecs/entities.rs
use crate::ecs::components::{
    CharacterClass, Equipment, Experience, Faction, Health, HealthComponent, MovementAI,
    MovementAIKind, Resource, Stats,
};
use common::PlayerClass;
use specs::{Entities, Entity, LazyUpdate, ReadExpect};

/// Creates a new player entity with all required components
// pub fn create_player_entity(
//     entities: &Entities,
//     lazy: &ReadExpect<LazyUpdate>,
//     player_name: String,
//     class: PlayerClass,
// ) -> Entity {
//     // Create the entity
//     let entity = entities.create();
//
//     lazy.insert(
//         entity,
//         CharacterClass {
//             class: class.clone(),
//         },
//     );
//
//     lazy.insert(entity, Faction::Player);
//
//     lazy.insert(
//         entity,
//         HealthComponent(Health::Alive {
//             hp: get_starting_hp(&class),
//             max_hp: get_starting_hp(&class),
//         },
//     ));
//
//     // Add more components as needed
//     lazy.insert(
//         entity,
//         Equipment {
//             weapon: None,
//             armor: None,
//         },
//     );
//
//     lazy.insert(entity, get_starting_stats(&class));
//
//     lazy.insert(
//         entity,
//         Resource {
//             mana: get_starting_mana(&class),
//             max_mana: get_starting_mana(&class),
//             stamina: 100,
//             max_stamina: 100,
//         },
//     );
//
//     lazy.insert(
//         entity,
//         Experience {
//             current: 0,
//             next_level: 100,
//         },
//     );
//
//     lazy.insert(
//         entity,
//         MovementAI {
//             kind: MovementAIKind::ExploreDungeon,
//         },
//     );
//
//     entity
// }

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
