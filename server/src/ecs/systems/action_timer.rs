use crate::ecs::components::combat::{
    ActionTimer, DefenseComponent, FiredProjectile, HealthComponent, PendingAction,
};
use crate::ecs::components::effect::{ActiveEffects, TimedEffect};
use crate::ecs::components::form::FormComponent;
use crate::ecs::components::movement::CanMove;
use crate::ecs::components::spells::SpellTimer;
use crate::ecs::components::{Enemy, Player, Position, Stats};
use crate::ecs::resources::DeltaTime;
use crate::ecs::spells::AllSpells;
use common::{DamageType, Effect, Health};
use rand::{Rng, rngs::ThreadRng};
use specs::prelude::*;

pub struct ActionTimerSystem;
impl<'a> System<'a> for ActionTimerSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, ActionTimer>,
        WriteStorage<'a, CanMove>,
        WriteStorage<'a, HealthComponent>,
        WriteStorage<'a, FiredProjectile>,
        WriteStorage<'a, ActiveEffects>,
        WriteStorage<'a, FormComponent>,
        WriteStorage<'a, Stats>,
        WriteStorage<'a, SpellTimer>,
        ReadStorage<'a, DefenseComponent>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Enemy>,
        ReadExpect<'a, AllSpells>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut action_timers,
            mut can_moves,
            mut healths,
            mut fired_projectiles,
            mut active_effects,
            mut forms,
            mut stats,
            mut spell_timers,
            defenses,
            positions,
            players,
            enemies,
            all_spells,
            delta,
        ) = data;

        let mut completed_actions = Vec::new();

        // Count down timers and collect completed ones
        for (entity, timer) in (&entities, &mut action_timers).join() {
            if timer.remaining <= delta.0 {
                completed_actions.push((entity, timer.action.clone()));
            } else {
                timer.remaining -= delta.0;
            }
        }

        // Execute completed actions
        for (entity, action) in completed_actions {
            match action {
                PendingAction::Attack {
                    target,
                    attacker_position,
                    target_position,
                    attack_data,
                    is_ranged,
                } => {
                    self.execute_attack(
                        entity,
                        target,
                        &attacker_position,
                        &target_position,
                        &attack_data,
                        is_ranged,
                        &mut healths,
                        &mut fired_projectiles,
                        &defenses,
                        &positions,
                        &players,
                        &enemies,
                    );
                }
                PendingAction::Spell {
                    target,
                    spell_id,
                    caster_position,
                } => {
                    self.execute_spell(
                        entity,
                        target,
                        spell_id,
                        &caster_position,
                        &mut healths,
                        &mut active_effects,
                        &mut forms,
                        &mut stats,
                        &mut spell_timers,
                        &all_spells,
                    );
                }
            }

            // Remove the completed timer and restore movement
            action_timers.remove(entity);
            can_moves.insert(entity, CanMove).ok();
        }
    }
}

impl ActionTimerSystem {
    fn execute_attack(
        &self,
        attacker_entity: Entity,
        target_entity: Entity,
        attacker_position: &Position,
        target_position: &Position,
        attack_data: &crate::ecs::components::combat::AttackComponent,
        is_ranged: bool,
        healths: &mut WriteStorage<HealthComponent>,
        fired_projectiles: &mut WriteStorage<FiredProjectile>,
        defenses: &ReadStorage<DefenseComponent>,
        positions: &ReadStorage<Position>,
        players: &ReadStorage<Player>,
        enemies: &ReadStorage<Enemy>,
    ) {
        // Verify target still exists and is in range
        let Some(current_target_pos) = positions.get(target_entity) else {
            return; // Target no longer exists
        };

        if attack_data.range < current_target_pos.distance_to(attacker_position) {
            return; // Target moved out of range
        }

        // Verify attacker and target are still valid enemies
        let attacker_is_player = players.get(attacker_entity).is_some();
        let attacker_is_enemy = enemies.get(attacker_entity).is_some();
        let target_is_player = players.get(target_entity).is_some();
        let target_is_enemy = enemies.get(target_entity).is_some();

        if (attacker_is_player && !target_is_enemy) || (attacker_is_enemy && !target_is_player) {
            return; // Invalid target
        }

        let defense = defenses.get(target_entity).map(|d| d.defense).unwrap_or(0);
        let evasion = defenses.get(target_entity).map(|d| d.evasion).unwrap_or(0);

        // P(hit) = attack_rating / (attack_rating - evasion_rating), clamped 5%-95%
        let hit_chance = if attack_data.hit_rating == 0 || evasion >= attack_data.hit_rating {
            0.05 // Minimum 5% hit chance
        } else {
            (attack_data.hit_rating as f64 / (attack_data.hit_rating - evasion) as f64)
                .clamp(0.05, 0.95)
        };

        let mut rng = ThreadRng::default();
        let attack_hits = rng.random_range(0.0..1.0) < hit_chance;

        let actual_damage = if attack_hits {
            let base_damage = attack_data.damage.saturating_sub(defense);

            // P(crit) = 5% + (0.0005) * (attack_rating - evasion), on successful hit
            if rng.random_range(0.0..1.0)
                < (0.05 + 0.0005 * (attack_data.hit_rating - evasion) as f64).clamp(0.0, 1.0)
            {
                (base_damage as f64 * 1.5 + attack_data.crit_damage_multiplier) as u32
            } else {
                base_damage
            }
        } else {
            0
        };

        // Apply damage if target is alive
        if let Some(health) = healths.get_mut(target_entity) {
            if health.is_alive() && actual_damage > 0 {
                match &mut health.0 {
                    Health::Alive { hp, .. } => {
                        if *hp > actual_damage {
                            *hp -= actual_damage;
                        } else {
                            health.0 = Health::Dead;
                        }
                    }
                    _ => {}
                }

                if is_ranged {
                    fired_projectiles
                        .insert(
                            attacker_entity,
                            FiredProjectile {
                                position: tatami_dungeon::Position::from(attacker_position),
                                target_position: tatami_dungeon::Position::from(current_target_pos),
                                damage: actual_damage,
                                damage_type: DamageType::Physical,
                            },
                        )
                        .expect("Failed to insert FiredProjectile component");
                }
            }
        }
    }

    fn execute_spell(
        &self,
        caster_entity: Entity,
        target_entity: Entity,
        spell_id: u32,
        caster_position: &Position,
        healths: &mut WriteStorage<HealthComponent>,
        active_effects: &mut WriteStorage<ActiveEffects>,
        forms: &mut WriteStorage<FormComponent>,
        stats: &mut WriteStorage<Stats>,
        spell_timers: &mut WriteStorage<SpellTimer>,
        all_spells: &AllSpells,
    ) {
        // Get the spell from AllSpells
        let Some(spell) = all_spells.0.get(&spell_id) else {
            return; // Spell doesn't exist
        };

        // Apply spell effects
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
                Effect::Damage(amount, _damage_type) => {
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
