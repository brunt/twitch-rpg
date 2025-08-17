use crate::ecs::components::combat::{ActionTimer, AttackComponent, AttackTarget, HealthComponent};
use crate::ecs::components::movement::{DesiredTargetPosition, MovementSpeed, TargetPosition};
use crate::ecs::components::{Enemy, Experience, Level, Player};
use crate::ecs::resources::Adventure;
use common::Health;
use specs::{Entities, Join, ReadExpect, ReadStorage, System, WriteStorage};

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
        WriteStorage<'a, ActionTimer>,
        WriteStorage<'a, Experience>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Enemy>,
        ReadExpect<'a, Option<Adventure>>,
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
            mut action_timers,
            mut experiences,
            players,
            enemies,
            adventure,
        ): Self::SystemData,
    ) {
        let Some(adventure) = adventure.as_ref() else {
            return;
        };
        for (entity, health) in (&entities, &healths).join() {
            if matches!(health.0, Health::Dead) {
                attack_targets.remove(entity);
                target_positions.remove(entity);
                attack_components.remove(entity);
                movements.remove(entity);
                desired_target_positions.remove(entity);
                action_timers.remove(entity);

                if enemies.get(entity).is_some() {
                    for (exp, _, _) in (&mut experiences, &players, &healths)
                        .join()
                        .filter(|(_, _, health)| !matches!(health.0, Health::Dead))
                    {
                        exp.current += 2 * adventure.difficulty.min(1);
                    }
                }
            }
        }
    }
}
