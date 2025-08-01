use specs::{Entities, Join, ReadStorage, System, WriteStorage};

use crate::ecs::components::Position;
use crate::ecs::components::movement::{CanMove, MovementSpeed, TargetPosition};

pub struct Movement;

impl<'a> System<'a> for Movement {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, MovementSpeed>,
        ReadStorage<'a, CanMove>,
        WriteStorage<'a, TargetPosition>,
    );

    fn run(&mut self, (entities, mut positions, speeds, can_move, mut targets): Self::SystemData) {
        let mut to_remove = Vec::new();

        for (entity, pos, speed, _, target) in
            (&entities, &mut positions, &speeds, &can_move, &mut targets).join()
        {
            let dx = target.x as i32 - pos.x as i32;
            let dy = target.y as i32 - pos.y as i32;

            // Already at the target
            if dx == 0 && dy == 0 {
                to_remove.push(entity);
                continue;
            }

            let dist = dx.abs().max(dy.abs()); // Chebyshev distance
            let steps = speed.0.min(dist as u32); // steps we can take this tick

            // Normalize direction to -1, 0, 1
            let dir_x = dx.signum();
            let dir_y = dy.signum();

            // Move step-by-step toward target, not exceeding speed
            let new_x = pos.x as i32 + dir_x * steps.min(dx.unsigned_abs()) as i32;
            let new_y = pos.y as i32 + dir_y * steps.min(dy.unsigned_abs()) as i32;

            pos.x = new_x as u32;
            pos.y = new_y as u32;
        }

        for entity in to_remove {
            targets.remove(entity);
        }
    }
}
