use crate::ecs::components::combat::{AttackComponent, HealthComponent};
use crate::ecs::components::effect::{ActiveEffects, TimedEffect};
use crate::ecs::components::form::FormComponent;

use crate::ecs::components::spells::{SpellCaster, SpellTarget, SpellTimer, Spellbook};
use crate::ecs::components::{Enemy, Player, Position, Stats};
use crate::ecs::spells::AllSpells;
use common::{Effect, Form, Health, Targeting};
use specs::prelude::*;

pub struct SpellcastingSystem;

impl<'a> System<'a> for SpellcastingSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, AllSpells>,
        ReadStorage<'a, SpellCaster>,
        ReadStorage<'a, SpellTarget>,
        ReadStorage<'a, Spellbook>,
        ReadStorage<'a, AttackComponent>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Enemy>,
        WriteStorage<'a, SpellTimer>,
        WriteStorage<'a, HealthComponent>,
        WriteStorage<'a, ActiveEffects>,
        WriteStorage<'a, FormComponent>,
        WriteStorage<'a, Stats>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            all_spells,
            spell_casters,
            spell_targets,
            spellbooks,
            attack_components,
            positions,
            players,
            enemies,
            mut spell_timers,
            mut healths,
            mut active_effects,
            mut forms,
            mut stats,
        ) = data;

        for (caster_entity, _, spell_target, spellbook, position) in (
            &entities,
            &spell_casters,
            &spell_targets,
            &spellbooks,
            &positions,
        )
            .join()
        {
            // Check if spell is on cooldown
            if let Some(timer) = spell_timers.get(caster_entity)
                && timer.remaining > 0.0
                && timer.spell_id == spell_target.spell_id
            {
                continue; // Still cooling down
            }

            // Get the spell from AllSpells
            let Some(spell) = all_spells.0.get(&spell_target.spell_id) else {
                continue; // Spell doesn't exist
            };

            // Check if caster has this spell in their spellbook
            if !spellbook.spells.contains(spell) {
                continue; // Caster doesn't know this spell
            }

            let target_entity = spell_target.entity;

            // Handle different targeting types
            match &spell.targeting {
                Targeting::Personal => {
                    // Target must be the caster themselves
                    if target_entity != caster_entity {
                        continue;
                    }
                    self.apply_spell_effects(
                        spell,
                        caster_entity,
                        target_entity,
                        &mut healths,
                        &mut active_effects,
                        &mut forms,
                        &mut stats,
                    );
                }
                Targeting::Single => {
                    // Check range to target
                    if let (Some(target_pos), Some(attack_comp)) = (
                        positions.get(target_entity),
                        attack_components.get(caster_entity),
                    ) {
                        if attack_comp.range < target_pos.distance_to(position) {
                            continue; // Out of range
                        }

                        // Validate target type (players can target enemies, enemies can target players)
                        let caster_is_player = players.get(caster_entity).is_some();
                        let target_is_player = players.get(target_entity).is_some();
                        let target_is_enemy = enemies.get(target_entity).is_some();

                        if caster_is_player && !target_is_enemy && target_entity != caster_entity {
                            continue; // Players can only target enemies or themselves
                        }
                        if !caster_is_player && !target_is_player {
                            continue; // Enemies can only target players
                        }

                        self.apply_spell_effects(
                            spell,
                            caster_entity,
                            target_entity,
                            &mut healths,
                            &mut active_effects,
                            &mut forms,
                            &mut stats,
                        );
                    } else {
                        continue; // No position or attack component
                    }
                }
                Targeting::PointRadius { radius } => {
                    // For AoE spells, we'll target all entities within radius of the target position
                    if let Some(target_pos) = positions.get(target_entity) {
                        // Check if caster can reach the target point
                        if let Some(attack_comp) = attack_components.get(caster_entity)
                            && attack_comp.range < target_pos.distance_to(position)
                        {
                            continue; // Out of range to cast at target point
                        }

                        // Apply effects to all entities within radius
                        for (nearby_entity, nearby_pos) in (&entities, &positions).join() {
                            if target_pos.distance_to(nearby_pos) <= *radius as u32 {
                                self.apply_spell_effects(
                                    spell,
                                    caster_entity,
                                    nearby_entity,
                                    &mut healths,
                                    &mut active_effects,
                                    &mut forms,
                                    &mut stats,
                                );
                            }
                        }
                    }
                }
                Targeting::Cone { angle: _, range } => {
                    // For now, implement as a simple range-based effect
                    // TODO: Implement proper cone targeting with angle calculations
                    if let Some(attack_comp) = attack_components.get(caster_entity)
                        && attack_comp.range < *range as u32
                    {
                        continue; // Out of range
                    }

                    self.apply_spell_effects(
                        spell,
                        caster_entity,
                        target_entity,
                        &mut healths,
                        &mut active_effects,
                        &mut forms,
                        &mut stats,
                    );
                }
                Targeting::Line { length, width: _ } => {
                    // For now, implement as a simple range-based effect
                    // TODO: Implement proper line targeting
                    if let Some(attack_comp) = attack_components.get(caster_entity)
                        && attack_comp.range < *length as u32
                    {
                        continue; // Out of range
                    }

                    self.apply_spell_effects(
                        spell,
                        caster_entity,
                        target_entity,
                        &mut healths,
                        &mut active_effects,
                        &mut forms,
                        &mut stats,
                    );
                }
            }

            // Set spell cooldown
            spell_timers
                .entry(caster_entity)
                .expect("failed to get spell timer entry")
                .or_insert_with(|| SpellTimer {
                    remaining: spell.cooldown,
                    spell_id: spell.id,
                });
        }
    }
}

impl SpellcastingSystem {
    fn apply_spell_effects(
        &self,
        spell: &common::Spell,
        caster_entity: Entity,
        target_entity: Entity,
        healths: &mut WriteStorage<HealthComponent>,
        active_effects: &mut WriteStorage<ActiveEffects>,
        forms: &mut WriteStorage<FormComponent>,
        stats: &mut WriteStorage<Stats>,
    ) {
        for (effect, duration) in &spell.effects {
            match effect {
                Effect::Heal(amount) => {
                    if let Some(health) = healths.get_mut(target_entity)
                        && let Health::Alive { hp, max_hp } = &mut health.0
                    {
                        *hp = (*hp + amount).min(*max_hp);
                    }
                }
                Effect::Revive => {
                    if let Some(health) = healths.get_mut(target_entity)
                        && matches!(health.0, Health::Dead)
                    {
                        health.0 = Health::Alive { hp: 1, max_hp: 1 };
                    }
                }
                Effect::Transform(transform_form) => {
                    if let Some(form) = forms.get_mut(target_entity) {
                        form.0 = transform_form.clone();
                    }

                    // Add timed effect if duration is specified
                    if let Some(duration) = duration {
                        active_effects
                            .entry(target_entity)
                            .expect("active effects entry")
                            .or_insert_with(ActiveEffects::default)
                            .effects
                            .push(TimedEffect {
                                effect: effect.clone(),
                                remaining_secs: *duration,
                            });
                    }
                }
                Effect::StatChange(_) => {
                    // StatAggregation system handles stat calculations
                    // We only add the effect to ActiveEffects if it has a duration
                    if let Some(duration) = duration {
                        active_effects
                            .entry(target_entity)
                            .expect("active effects entry")
                            .or_insert_with(ActiveEffects::default)
                            .effects
                            .push(TimedEffect {
                                effect: effect.clone(),
                                remaining_secs: *duration,
                            });
                    }
                }
                Effect::AttackModifier(_) => {
                    // Similar to StatChange, handled by other systems
                    if let Some(duration) = duration {
                        active_effects
                            .entry(target_entity)
                            .expect("active effects entry")
                            .or_insert_with(ActiveEffects::default)
                            .effects
                            .push(TimedEffect {
                                effect: effect.clone(),
                                remaining_secs: *duration,
                            });
                    }
                }
                Effect::DefenseModifier(_) => {
                    // Similar to StatChange, handled by other systems
                    if let Some(duration) = duration {
                        active_effects
                            .entry(target_entity)
                            .expect("active effects entry")
                            .or_insert_with(ActiveEffects::default)
                            .effects
                            .push(TimedEffect {
                                effect: effect.clone(),
                                remaining_secs: *duration,
                            });
                    }
                }
                Effect::Damage(amount, damage_type) => {
                    if let Some(health) = healths.get_mut(target_entity)
                        && let Health::Alive { hp, .. } = &mut health.0
                    {
                        if *hp > *amount {
                            *hp -= amount;
                        } else {
                            health.0 = Health::Dead;
                        }
                    }
                    // TODO: Add projectile visual for damage spells similar to combat system
                }
            }
        }
    }
}
