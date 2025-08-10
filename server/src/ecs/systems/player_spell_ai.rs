use crate::ecs::components::combat::HealthComponent;
use crate::ecs::components::effect::ActiveEffects;
use crate::ecs::components::spells::{SpellCaster, SpellTarget, SpellTimer, Spellbook};
use crate::ecs::components::{Enemy, Player, Position};
use crate::ecs::resources::Adventure;
use crate::ecs::resources::RoomCheck;
use common::{Health, SpellCasterRestriction, Targeting};
use specs::prelude::*;

pub struct PlayerSpellAISystem;

impl<'a> System<'a> for PlayerSpellAISystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Enemy>,
        ReadStorage<'a, HealthComponent>,
        ReadStorage<'a, SpellCaster>,
        ReadStorage<'a, Spellbook>,
        ReadStorage<'a, SpellTimer>,
        ReadStorage<'a, ActiveEffects>,
        WriteStorage<'a, SpellTarget>,
        ReadExpect<'a, Option<Adventure>>,
    );

    fn run(
        &mut self,
        (
            entities,
            positions,
            players,
            enemies,
            healths,
            spell_casters,
            spellbooks,
            spell_timers,
            active_effects,
            mut spell_targets,
            adv,
        ): Self::SystemData,
    ) {
        let Some(adv) = adv.as_ref() else {
            return;
        };

        for (player_entity, pos, _, health) in (&entities, &positions, &players, &healths)
            .join()
            .filter(|(_, _, _, health)| !matches!(health.0, Health::Dead))
        {
            let Some(current_room) = adv
                .get_current_floor()
                .rooms
                .iter()
                .find(|r| r.id == adv.current_room_index)
            else {
                return;
            };

            // Check if player has spellcasting capability
            let (Some(_), Some(spellbook)) = (
                spell_casters.get(player_entity),
                spellbooks.get(player_entity),
            ) else {
                continue;
            };

            // Check if not on spell cooldown
            let not_on_cooldown = spell_timers
                .get(player_entity)
                .map_or(true, |timer| timer.remaining <= 0.0);

            if !not_on_cooldown {
                continue;
            }

            // Find enemies in the current room
            let enemies_in_room: Vec<(Entity, &Position)> =
                (&entities, &enemies, &healths, &positions)
                    .join()
                    .filter(|(_, _, health, pos)| {
                        !matches!(health.0, Health::Dead)
                            && current_room.contains(&tatami_dungeon::Position::from(*pos))
                    })
                    .map(|(enemy, _, _, pos)| (enemy, pos))
                    .collect();

            // Get currently active effects on the player
            let current_effects = active_effects.get(player_entity);

            let has_enemies = !enemies_in_room.is_empty();

            // Priority 1: Personal spells that aren't already active
            let preferred_spell = spellbook
                .spells
                .iter()
                .filter(|s| {
                    matches!(s.targeting, Targeting::Personal)
                        && matches!(
                            s.caster_restriction,
                            SpellCasterRestriction::PlayerClass { .. }
                        )
                })
                .find(|spell| {
                    // Check if any of this spell's effects are already active
                    if let Some(effects) = current_effects {
                        !spell.effects.iter().any(|(effect, _duration)| {
                            effects
                                .effects
                                .iter()
                                .any(|active_effect| &active_effect.effect == effect)
                        })
                    } else {
                        true // No active effects, so spell can be cast
                    }
                })
                .or_else(|| {
                    // Priority 2: Critical healing when health is very low
                    let should_heal = if let Health::Alive { hp, max_hp } = health.0 {
                        hp < max_hp / 3 // Heal when below 1/3 health
                    } else {
                        false
                    };

                    if should_heal {
                        spellbook
                            .spells
                            .iter()
                            .filter(|s| {
                                matches!(s.targeting, Targeting::Personal)
                                    && matches!(
                                        s.caster_restriction,
                                        SpellCasterRestriction::PlayerClass { .. }
                                    )
                            })
                            .next()
                    } else {
                        None
                    }
                })
                .or_else(|| {
                    // Priority 3: Offensive spells when enemies are present
                    if has_enemies {
                        spellbook
                            .spells
                            .iter()
                            .filter(|s| {
                                !matches!(s.targeting, Targeting::Personal)
                                    && matches!(
                                        s.caster_restriction,
                                        SpellCasterRestriction::PlayerClass { .. }
                                    )
                            })
                            .next()
                    } else {
                        None
                    }
                })
                .or_else(|| {
                    // Priority 4: Personal spells when no enemies (for minor healing/buffs)
                    let should_use_personal = if let Health::Alive { hp, max_hp } = health.0 {
                        hp < max_hp / 2 // Use personal spells when below half health
                    } else {
                        false
                    };

                    if should_use_personal {
                        spellbook
                            .spells
                            .iter()
                            .filter(|s| {
                                matches!(s.targeting, Targeting::Personal)
                                    && matches!(
                                        s.caster_restriction,
                                        SpellCasterRestriction::PlayerClass { .. }
                                    )
                            })
                            .next()
                    } else {
                        None
                    }
                });

            // Cast the selected spell
            if let Some(spell) = preferred_spell {
                let target_entity = match spell.targeting {
                    Targeting::Personal => player_entity,
                    _ => {
                        // For non-personal spells, target the first enemy in room
                        if let Some((enemy_id, _)) = enemies_in_room.first() {
                            *enemy_id
                        } else {
                            dbg!();
                            continue; // No valid target for offensive spell
                        }
                    }
                };

                spell_targets
                    .insert(
                        player_entity,
                        SpellTarget {
                            entity: target_entity,
                            spell_id: spell.id,
                        },
                    )
                    .expect("failed to set spell target");
            }
        }
    }
}
