use specs::{Join, ReadStorage, System, WriteStorage};

use crate::ecs::components::{MovementSpeed, Position, TargetPosition};

pub struct Movement;

impl<'a> System<'a> for Movement {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, MovementSpeed>,
        ReadStorage<'a, TargetPosition>,
    );

    fn run(&mut self, (mut positions, speed, target): Self::SystemData) {
        for (pos, speed, target) in (&mut positions, &speed, &target).join() {
            // Calculate direction vector (dx, dy)
            let dx = (target.x - pos.x).abs();
            let dy = (target.y - pos.y).abs();

            // If already at target, skip
            if dx == 0 && dy == 0 {
                continue;
            }

            // distance formula (chebyshev)
            let dist = dx.max(dy);

            // Compute step in x and y, limited by speed
            let step_x = if dx > 0 {
                dx.min(speed.0)
            } else {
                dx.max(-speed.0)
            };

            let step_y = if dy > 0 {
                dy.min(speed.0)
            } else {
                dy.max(-speed.0)
            };

            if dist <= *speed {
                pos.x = target.x;
                pos.y = target.y;
                continue;
            }

            //TODO: finish this
            pos.x = pos.x + step_x;
            pos.y = pos.y + step_y;

            // dbg!(pos.x, pos.y);
        }
    }
}
