use crate::ecs::components::Position;
use crate::ecs::components::movement::{TargetPosition, Wander};
use crate::ecs::resources::Adventure;
use rand::{Rng, SeedableRng};
use specs::{Entities, Join, ReadExpect, ReadStorage, System, WriteStorage};

pub struct RandomWander;

impl<'a> System<'a> for RandomWander {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Wander>,
        WriteStorage<'a, TargetPosition>,
        ReadExpect<'a, Option<Adventure>>,
    );

    fn run(&mut self, (entities, positions, _, mut targets, adventure): Self::SystemData) {
        let mut rng = rand::rngs::SmallRng::from_os_rng();

        let Some(adventure) = adventure.as_ref() else {
            return;
        };

        let floor = &adventure.dungeon.floors[adventure.current_floor_index];
        let max_x = floor.tiles[0].len() as i32;
        let max_y = floor.tiles.len() as i32;

        for (entity, pos) in (&entities, &positions).join() {
            if targets.get(entity).is_none() {
                let dx = rng.random_range(-1..=1);
                let dy = rng.random_range(-1..=1);

                let new_x = (pos.x as i32 + dx).clamp(0, max_x - 1);
                let new_y = (pos.y as i32 + dy).clamp(0, max_y - 1);

                targets
                    .insert(
                        entity,
                        TargetPosition {
                            x: new_x as u32,
                            y: new_y as u32,
                        },
                    )
                    .expect("Failed to insert TargetPosition");
            }
        }
    }
}
