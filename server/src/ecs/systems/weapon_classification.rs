use specs::prelude::*;
use common::{EquipmentSlot, Item};
use crate::ecs::components::combat::{MeleeAttacker, RangedAttacker};
use crate::ecs::components::inventory::Equipment;

/// System for giving players a ranged or melee marker component based on their main-hand weapon.
/// A weapon with more than 1 range bonus is considered a Ranged Weapon.
pub struct WeaponClassifierSystem;

impl<'a> System<'a> for WeaponClassifierSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Equipment>,
        WriteStorage<'a, RangedAttacker>,
        WriteStorage<'a, MeleeAttacker>,
    );

    fn run(&mut self, (entities, equipment, mut ranged, mut melee): Self::SystemData) {
        for (entity, eq) in (&entities, &equipment).join() {
            let ranged_weapon = eq.slots
                .get(&EquipmentSlot::MainHand)
                .and_then(|item| item.stats.as_ref())
                .and_then(|stats| stats.attack_modifiers.as_ref())
                .map_or(false, |atk| atk.range_bonus > 1);

            if ranged_weapon {
                ranged.insert(entity, RangedAttacker).ok();
                melee.remove(entity);
            } else {
                melee.insert(entity, MeleeAttacker).ok();
                ranged.remove(entity);
            }
        }
    }
}
