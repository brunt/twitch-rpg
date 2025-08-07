use crate::ecs::components::combat::{
    AttackComponent, AttackTarget, AttackTimer, DefenseComponent, FiredProjectile, HealthComponent,
    MeleeAttacker, RangedAttacker,
};
use crate::ecs::components::movement::{CanMove, MovementSpeed};
use crate::ecs::components::{Enemy, Player, Position};
use crate::ecs::resources::DeltaTime;
use common::{DamageType, Health};
use rand::Rng;
use specs::prelude::*;

pub struct CombatSystem;

impl<'a> System<'a> for CombatSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, AttackComponent>,
        ReadStorage<'a, AttackTarget>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Enemy>,
        ReadStorage<'a, MeleeAttacker>,
        ReadStorage<'a, RangedAttacker>,
        ReadStorage<'a, DefenseComponent>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, HealthComponent>,
        WriteStorage<'a, FiredProjectile>,
        WriteStorage<'a, AttackTimer>,
        WriteStorage<'a, CanMove>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            attacks,
            targets,
            players,
            enemies,
            melee,
            range,
            defenses,
            positions,
            mut healths,
            mut fired_projectiles,
            mut attack_timers,
            mut can_move,
        ) = data;

        for (attacker_entity, attack, attack_target, position) in
            (&entities, &attacks, &targets, &positions).join()
        {
            // Check if attacker is on cooldown
            if let Some(timer) = attack_timers.get(attacker_entity) {
                if timer.remaining > 0.0 {
                    continue; // Skip this tick, still cooling down
                }
            }

            // Attacker must be Player or Enemy
            let attacker_is_player = players.get(attacker_entity).is_some();
            let attacker_is_enemy = enemies.get(attacker_entity).is_some();

            let target_entity = attack_target.entity;
            let target_is_player = players.get(target_entity).is_some();
            let target_is_enemy = enemies.get(target_entity).is_some();

            if (attacker_is_player && !target_is_enemy) || (attacker_is_enemy && !target_is_player)
            {
                continue;
            }

            let Some(target_pos) = positions.get(target_entity) else {
                continue;
            };

            if attack.range < target_pos.distance_to(position) {
                continue; // Out of range
            }

            let defense = defenses.get(target_entity).map(|d| d.defense).unwrap_or(0);
            let evasion = defenses.get(target_entity).map(|d| d.evasion).unwrap_or(0);

            let actual_damage = attack.damage.saturating_sub(defense);

            // this component gets re-added in the attack_cooldown system
            can_move.remove(attacker_entity);

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

                    if range.get(attacker_entity).is_some() {
                        fired_projectiles
                            .insert(
                                attacker_entity,
                                FiredProjectile {
                                    position: tatami_dungeon::Position::from(position),
                                    target_position: tatami_dungeon::Position::from(target_pos),
                                    damage: actual_damage,
                                    damage_type: DamageType::Purple,
                                },
                            )
                            .expect("Failed to insert FiredProjectile component");
                    }
                }
            }

            // Only insert a cooldown timer if one does not exist yet
            attack_timers
                .entry(attacker_entity)
                .expect("failed to get entry for attacker")
                .or_insert_with(|| AttackTimer {
                    //TODO: u32 or f64 for counting time?
                    remaining: attack.cooldown as f64 / 1000.0,
                });
        }
    }
}
