use crate::ecs::components::combat::{AttackTarget, HealthComponent};
use crate::ecs::components::movement::TargetPosition;
use crate::ecs::components::spells::{SpellCaster, SpellTarget, SpellTimer, Spellbook};
use crate::ecs::components::{Enemy, Player, Position};
use crate::ecs::spells::AllSpells;
use common::{Health, SpellCasterRestriction, Targeting};
use specs::Entity;
use specs::{Entities, Join, ReadExpect, ReadStorage, System, WriteStorage};

pub struct EnemyAISystem;

impl<'a> System<'a> for EnemyAISystem {
    type SystemData = (
        ReadStorage<'a, Enemy>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, HealthComponent>,
        ReadStorage<'a, SpellCaster>,
        ReadStorage<'a, Spellbook>,
        ReadStorage<'a, SpellTimer>,
        WriteStorage<'a, TargetPosition>,
        WriteStorage<'a, AttackTarget>,
        WriteStorage<'a, SpellTarget>,
        ReadExpect<'a, AllSpells>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (
            enemies,
            players,
            positions,
            healths,
            spell_casters,
            spellbooks,
            spell_timers,
            mut targets,
            mut attack_targets,
            mut spell_targets,
            all_spells,
            entities,
        ): Self::SystemData,
    ) {
        let alive_players: Vec<(Entity, &Position)> = (&entities, &players, &positions, &healths)
            .join()
            .filter(|(_, _, _, health)| !matches!(health.0, Health::Dead))
            .map(|(entity, _, pos, _)| (entity, pos))
            .collect();

        for (enemy_pos, _, enemy_entity) in (&positions, &enemies, &entities).join() {
            // Find closest alive player
            if let Some((closest_player_entity, closest_player_pos)) = alive_players
                .iter()
                .min_by_key(|(_, player_pos)| enemy_pos.distance_to(player_pos))
            {
                // Check if enemy can cast spells and should prioritize spellcasting
                let should_cast_spell = if let (Some(_), Some(spellbook)) = (
                    spell_casters.get(enemy_entity),
                    spellbooks.get(enemy_entity),
                ) {
                    // Check if not on spell cooldown
                    let not_on_cooldown = spell_timers
                        .get(enemy_entity)
                        .map_or(true, |timer| timer.remaining <= 0.0);

                    if not_on_cooldown {
                        // Find a suitable spell for any target
                        spellbook.spells.iter().any(|spell| {
                            matches!(
                                spell.caster_restriction,
                                SpellCasterRestriction::Enemy | SpellCasterRestriction::All
                            ) && matches!(
                                spell.targeting,
                                Targeting::Single
                                    | Targeting::PointRadius { .. }
                                    | Targeting::Personal
                                    | Targeting::Cone { .. }
                                    | Targeting::Line { .. }
                            )
                        })
                    } else {
                        false
                    }
                } else {
                    false
                };

                if should_cast_spell {
                    // Find the best spell to cast
                    if let Some(spellbook) = spellbooks.get(enemy_entity) {
                        // Check if enemy should prioritize healing (if health is low)
                        let enemy_health = healths.get(enemy_entity);
                        let should_heal = if let Some(health) = enemy_health {
                            if let Health::Alive { hp, max_hp } = health.0 {
                                hp < max_hp / 3 // Heal when below 1/3 health
                            } else {
                                false
                            }
                        } else {
                            false
                        };

                        let preferred_spell = if should_heal {
                            // Look for personal/healing spells first
                            spellbook
                                .spells
                                .iter()
                                .filter(|s| {
                                    matches!(s.targeting, Targeting::Personal)
                                        && matches!(
                                            s.caster_restriction,
                                            SpellCasterRestriction::Enemy
                                                | SpellCasterRestriction::All
                                        )
                                })
                                .next()
                                .or_else(|| {
                                    // Fallback to offensive spells
                                    spellbook
                                        .spells
                                        .iter()
                                        .filter(|s| {
                                            !matches!(s.targeting, Targeting::Personal)
                                                && matches!(
                                                    s.caster_restriction,
                                                    SpellCasterRestriction::Enemy
                                                        | SpellCasterRestriction::All
                                                )
                                        })
                                        .next()
                                })
                        } else {
                            // Look for offensive spells first
                            spellbook
                                .spells
                                .iter()
                                .filter(|s| {
                                    !matches!(s.targeting, Targeting::Personal)
                                        && matches!(
                                            s.caster_restriction,
                                            SpellCasterRestriction::Enemy
                                                | SpellCasterRestriction::All
                                        )
                                })
                                .next()
                                .or_else(|| {
                                    // Fallback to personal spells
                                    spellbook
                                        .spells
                                        .iter()
                                        .filter(|s| {
                                            matches!(s.targeting, Targeting::Personal)
                                                && matches!(
                                                    s.caster_restriction,
                                                    SpellCasterRestriction::Enemy
                                                        | SpellCasterRestriction::All
                                                )
                                        })
                                        .next()
                                })
                        };

                        if let Some(spell) = preferred_spell {
                            let target_entity = match spell.targeting {
                                Targeting::Personal => enemy_entity,
                                _ => *closest_player_entity, // All other targeting types target the player
                            };

                            spell_targets
                                .insert(
                                    enemy_entity,
                                    SpellTarget {
                                        entity: target_entity,
                                        spell_id: spell.id,
                                    },
                                )
                                .expect("failed to set enemy spell target");
                        }
                    }
                } else {
                    // Default behavior: set attack target and move toward player
                    attack_targets
                        .insert(
                            enemy_entity,
                            AttackTarget {
                                entity: *closest_player_entity,
                            },
                        )
                        .expect("failed to set enemy attack target");

                    // Set the target to the player's current position
                    targets
                        .insert(enemy_entity, TargetPosition::from(*closest_player_pos))
                        .expect("Failed to insert enemy target position");
                }
            }
        }
    }
}
