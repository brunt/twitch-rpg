use crate::ecs::components::Position;
use crate::ecs::components::combat::{ActionTimer, AttackComponent, PendingAction};
use crate::ecs::components::movement::CanMove;
use crate::ecs::components::spells::{SpellCaster, SpellTarget};
use crate::ecs::spells::AllSpells;
use common::TargetFilter;
use specs::prelude::*;

// system for creating spell buildup timers
pub struct SpellcastingSystem;

impl<'a> System<'a> for SpellcastingSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, AllSpells>,
        ReadStorage<'a, SpellCaster>,
        WriteStorage<'a, SpellTarget>,
        ReadStorage<'a, AttackComponent>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, ActionTimer>,
        WriteStorage<'a, CanMove>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            all_spells,
            spell_casters,
            mut spell_targets,
            attack_components,
            positions,
            mut action_timers,
            mut can_move,
        ) = data;

        let mut to_remove = Vec::new();
        for (caster_entity, _, spell_target, position) in
            (&entities, &spell_casters, &spell_targets, &positions).join()
        {
            // Check if caster already has a pending action
            if action_timers.get(caster_entity).is_some() {
                continue; // Skip this tick, already preparing an action
            }
            // Get the spell from AllSpells
            let Some(spell) = all_spells.0.get(&spell_target.spell_id) else {
                continue; // Spell doesn't exist
            };

            let target_entity = spell_target.target;

            // Basic validation - more detailed validation will happen when buildup completes
            match spell.targeting.filter {
                TargetFilter::SelfOnly => {
                    if target_entity != caster_entity {
                        continue;
                    }
                }
                TargetFilter::AllyOrSelf => {
                    // buffs have an implicit minimum range that offensive spells don't
                    if let (Some(target_pos), Some(attack_comp)) = (
                        positions.get(target_entity),
                        attack_components.get(caster_entity),
                    ) {
                        if attack_comp.range.min(3) < target_pos.distance_to(position) {
                            continue;
                        }
                    } else {
                        continue; // No position or attack component
                    }
                }
                TargetFilter::EnemyOnly => {
                    if let (Some(target_pos), Some(attack_comp)) = (
                        positions.get(target_entity),
                        attack_components.get(caster_entity),
                    ) {
                        if attack_comp.range < target_pos.distance_to(position) {
                            continue;
                        }
                    } else {
                        continue; // No position or attack component
                    }
                }
                TargetFilter::Any => unimplemented!(),
            }

            // Remove movement capability during buildup
            can_move.remove(caster_entity);

            // Create action timer with spell buildup time
            action_timers
                .insert(
                    caster_entity,
                    ActionTimer {
                        remaining: spell.cooldown, // Using cooldown as buildup time
                        action: PendingAction::Spell {
                            target: target_entity,
                            spell_id: spell_target.spell_id,
                            caster_position: *position,
                        },
                    },
                )
                .expect("Failed to insert ActionTimer component");

            to_remove.push(caster_entity);
        }

        for entity in to_remove {
            spell_targets.remove(entity);
        }
    }
}
