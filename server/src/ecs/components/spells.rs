use crate::ecs::components::Component;
use crate::ecs::components::DenseVecStorage;
use crate::ecs::components::NullStorage;
use crate::ecs::spells::AllSpells;
use common::PlayerClass;
use common::Spell;
use common::SpellCasterRestriction;
use serde::Deserialize;
use specs::Entity;
use std::collections::HashSet;

// spells refer to any abilities other than basic attacks
#[derive(Debug, Component, Clone)]
pub struct Spellbook {
    pub spells: HashSet<Spell>,
}

impl Spellbook {
    pub fn from_class_and_level(
        class: Option<PlayerClass>,
        level: u32,
        all_spells: &AllSpells,
    ) -> Self {
        let mut spells = HashSet::new();

        for spell in all_spells.0.values() {
            match &spell.caster_restriction {
                SpellCasterRestriction::All => {
                    spells.insert(spell.clone());
                }
                SpellCasterRestriction::Enemy => {
                    if class.is_none() {
                        spells.insert(spell.clone());
                    }
                }
                SpellCasterRestriction::PlayerClass { classes, min_level } => {
                    if let Some(player_class) = class {
                        if classes.contains(&player_class) {
                            if min_level.map_or(true, |min| level >= min) {
                                spells.insert(spell.clone());
                            }
                        }
                    }
                }
            }
        }

        Spellbook { spells }
    }
}

/// The entity that this entity is targeting for spell casting
#[derive(Component)]
pub struct SpellTarget {
    pub caster: Entity,
    pub target: Entity,
    pub spell_id: u32,
}

/// Tracks spell cooldowns for entities
#[derive(Component, Default)]
pub struct SpellTimer {
    pub remaining: f64,
    pub spell_id: u32,
}

/// Marker component indicating this entity can cast spells
#[derive(Component)]
#[storage(NullStorage)]
pub struct SpellCaster;

// pub struct SpellEffect {
//     pub effect: Effect,
//     pub target: Option<Entity>,
// }
