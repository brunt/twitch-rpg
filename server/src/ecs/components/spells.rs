use crate::ecs::components::Component;
use crate::ecs::components::DenseVecStorage;
use common::PlayerClass;
use common::Spell;
use serde::Deserialize;
use specs::Entity;
use std::collections::HashSet;

// spells refer to any abilities other than basic attacks
#[derive(Debug, Component, Clone)]
pub struct Spellbook {
    pub spells: HashSet<Spell>,
}

impl Spellbook {
    pub fn from_class(class: PlayerClass, all_spells: &AllSpells) -> Self {
        let mut spells = HashSet::new();
        match class {
            PlayerClass::Wizard => {
                // I need to populate this with spells from the ECS resource AllSpells
                // spells.insert()
            }
            _ => {}
        }
        Spellbook { spells }
    }
}

// pub struct SpellEffect {
//     pub effect: Effect,
//     pub target: Option<Entity>,
// }
