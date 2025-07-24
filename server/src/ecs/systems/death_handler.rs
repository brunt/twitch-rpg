use crate::ecs::components::combat::{AttackTarget, HealthComponent};
use crate::ecs::components::movement::{MovementSpeed, TargetPosition};
use common::Health;
use specs::{Entities, Join, ReadStorage, System, WriteStorage};

pub struct DeathCleanupSystem;

impl<'a> System<'a> for DeathCleanupSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, HealthComponent>,
        WriteStorage<'a, AttackTarget>,
        WriteStorage<'a, TargetPosition>,
        WriteStorage<'a, MovementSpeed>,
    );

    fn run(
        &mut self,
        (entities, healths, mut attack_targets, mut target_positions, mut movements): Self::SystemData,
    ) {
        for (entity, health) in (&entities, &healths).join() {
            if matches!(health.0, Health::Dead) {
                attack_targets.remove(entity);
                target_positions.remove(entity);
                movements.remove(entity);
            }
        }
    }
}
