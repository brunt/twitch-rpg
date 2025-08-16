use crate::ecs::components::combat::{
    ActionTimer, AttackComponent, AttackTarget, MeleeAttacker, PendingAction, RangedAttacker,
};
use crate::ecs::components::movement::CanMove;
use crate::ecs::components::{Enemy, Player, Position};

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
        ReadStorage<'a, Position>,
        WriteStorage<'a, ActionTimer>,
        WriteStorage<'a, CanMove>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            attacks,
            targets,
            players,
            enemies,
            _melee,
            range,
            positions,
            mut action_timers,
            mut can_move,
        ) = data;

        for (attacker_entity, attack, attack_target, position) in
            (&entities, &attacks, &targets, &positions).join()
        {
            // Check if attacker already has a pending action
            if action_timers.get(attacker_entity).is_some() {
                continue; // Skip this tick, already preparing an action
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

            // Create pending action - damage calculation happens when buildup completes
            let is_ranged = range.get(attacker_entity).is_some();

            // Remove movement capability during buildup
            can_move.remove(attacker_entity);

            // Create action timer with buildup time
            action_timers
                .insert(
                    attacker_entity,
                    ActionTimer {
                        remaining: attack.cooldown as f64 / 1000.0, // Using cooldown as buildup time
                        action: PendingAction::Attack {
                            target: target_entity,
                            attacker_position: *position,
                            target_position: *target_pos,
                            attack_data: attack.clone(),
                            is_ranged,
                        },
                    },
                )
                .expect("Failed to insert ActionTimer component");
        }
    }
}
