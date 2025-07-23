use specs::prelude::*;
use rand::Rng;
use common::Health;
use crate::ecs::components::combat::{AttackComponent, AttackTarget, DefenseComponent, HealthComponent, MeleeAttacker};
use crate::ecs::components::{Enemy, Player, Position};
use crate::ecs::resources::DeltaTime;

pub struct CombatSystem;

impl<'a> System<'a> for CombatSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, AttackComponent>,
        ReadStorage<'a, AttackTarget>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Enemy>,
        ReadStorage<'a, MeleeAttacker>,
        ReadStorage<'a, DefenseComponent>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, HealthComponent>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            attacks,
            targets,
            players,
            enemies,
            melee,
            defenses,
            positions,
            mut healths,
            delta_time,
        ) = data;

        for (attacker_entity, attack, attack_target, position) in (&entities, &attacks, &targets, &positions).join() {
            // Attacker must be Player or Enemy
            let attacker_is_player = players.get(attacker_entity).is_some();
            let attacker_is_enemy = enemies.get(attacker_entity).is_some();

            let target_entity = attack_target.entity;
            // Target must be Player or Enemy (opposite kind)
            let target_is_player = players.get(target_entity).is_some();
            let target_is_enemy = enemies.get(target_entity).is_some();

            // Enforce opposing teams
            if (attacker_is_player && !target_is_enemy) ||
                (attacker_is_enemy && !target_is_player) {
                continue;
            }
            
            // attacker must have attack range to hit target
            let Some(target_pos) = positions.get(target_entity) else { continue };
            if attack.range < target_pos.distance_to(position) {
                // Out of range
                continue;
            }

            // Retrieve target's defense and health
            let defense = defenses.get(target_entity).map(|d| d.defense).unwrap_or(0);
            let evasion = defenses.get(target_entity).map(|d| d.evasion).unwrap_or(0);

            // Simple hit/evasion check
            let hit_roll: u32 = rand::thread_rng().gen_range(0..100);
            if hit_roll < evasion {
                // Missed attack
                continue;
            }

            let actual_damage = attack.damage.saturating_sub(defense);

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
                }
            }
        }
    }
}
