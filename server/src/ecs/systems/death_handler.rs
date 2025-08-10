use crate::ecs::components::combat::{AttackComponent, AttackTarget, HealthComponent};
use crate::ecs::components::movement::{DesiredTargetPosition, MovementSpeed, TargetPosition};
use common::Health;
use specs::{Entities, Join, ReadStorage, System, WriteStorage};

pub struct DeathCleanupSystem;

impl<'a> System<'a> for DeathCleanupSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, HealthComponent>,
        WriteStorage<'a, AttackTarget>,
        WriteStorage<'a, AttackComponent>,
        WriteStorage<'a, TargetPosition>,
        WriteStorage<'a, MovementSpeed>,
        WriteStorage<'a, DesiredTargetPosition>,
    );

    fn run(
        &mut self,
        (
            entities,
            healths,
            mut attack_targets,
            mut attack_components,
            mut target_positions,
            mut movements,
            mut desired_target_positions,
        ): Self::SystemData,
    ) {
        for (entity, health) in (&entities, &healths).join() {
            if matches!(health.0, Health::Dead) {
                attack_targets.remove(entity);
                target_positions.remove(entity);
                attack_components.remove(entity);
                movements.remove(entity);
                desired_target_positions.remove(entity);
            }
        }
    }
}
