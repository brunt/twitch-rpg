use crate::ecs::components::class::CharacterClass;
use crate::ecs::components::effect::ActiveEffects;
use crate::ecs::components::inventory::Equipment;
use crate::ecs::components::{Player, Stats};
use common::Effect;
use specs::{Entities, Join, ReadStorage, System, WriteStorage};

pub struct StatAggregation;

impl<'a> System<'a> for StatAggregation {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, CharacterClass>,
        ReadStorage<'a, Equipment>,
        ReadStorage<'a, ActiveEffects>,
        WriteStorage<'a, Stats>,
    );

    fn run(
        &mut self,
        (entities, players, classes, equipment, active_effects, mut stats): Self::SystemData,
    ) {
        // This system iterates over all players and recalculates their stats for the current frame
        // based on their class, equipped items, and any temporary buffs or debuffs.
        for (entity, _, class, equipment) in (&entities, &players, &classes, &equipment).join() {
            // 1. Start with the character's base stats from their class.
            let mut aggregated_stats = Stats::new(&class.0);

            // 2. Add stats from all equipped items.
            for item in equipment.slots.values() {
                if let Some(item_stats) = &item.stats {
                    aggregated_stats.strength += item_stats.strength.unwrap_or(0);
                    aggregated_stats.agility += item_stats.agility.unwrap_or(0);
                    aggregated_stats.intelligence += item_stats.intelligence.unwrap_or(0);
                }
            }

            // 3. Calculate the total delta from any active temporary effects (e.g., potions).
            // These are handled as signed integers because effects can be negative (debuffs).
            let mut str_delta = 0i32;
            let mut agi_delta = 0i32;
            let mut int_delta = 0i32;

            if let Some(effects) = active_effects.get(entity) {
                for timed_effect in &effects.effects {
                    if let Effect::StatChange(stat_change) = &timed_effect.effect {
                        str_delta += stat_change.strength.unwrap_or(0);
                        agi_delta += stat_change.agility.unwrap_or(0);
                        int_delta += stat_change.intelligence.unwrap_or(0);
                    }
                }
            }

            // 4. Apply the final deltas to the combined base + equipment stats and update
            // the entity's Stats component. We clamp at 0 to prevent stats from going negative.
            if let Some(entity_stats) = stats.get_mut(entity) {
                entity_stats.strength =
                    (aggregated_stats.strength as i32 + str_delta).max(0) as u32;
                entity_stats.agility = (aggregated_stats.agility as i32 + agi_delta).max(0) as u32;
                entity_stats.intelligence =
                    (aggregated_stats.intelligence as i32 + int_delta).max(0) as u32;
            }
        }
    }
}
