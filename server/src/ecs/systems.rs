use specs::{Dispatcher, Join, ReadStorage, System, World, WorldExt, WriteStorage};
use specs_derive::Component;
use crate::ecs::components;
use crate::ecs::components::{MovementSpeed, Position, TargetPosition};

struct Movement;

impl<'a> System<'a> for Movement {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, MovementSpeed>,
        ReadStorage<'a, TargetPosition>,
    );

    fn run(&mut self, (mut positions, speed, target): Self::SystemData) {
        for (pos, speed, target) in (&mut positions, &speed, target).join() {
            // Calculate direction vector (dx, dy)
            let dx = (target.0 as i32 - pos.0 as i32).abs() as u32;
            let dy = (target.1 as i32 - pos.1 as i32).abs() as u32;

            // If already at target, skip
            if dx == 0 && dy == 0 {
                continue;
            }
            
            // distance formula (chebyshev)
            let dist = dx.max(dy);

            // Compute step in x and y, limited by speed
            let step_x = if dx > 0 {
                dx.min(speed.0 as i32)
            } else {
                dx.max(-(speed.0 as i32))
            };

            let step_y = if dy > 0 {
                dy.min(speed.0 as i32)
            } else {
                dy.max(-(speed.0 as i32))
            };
            
            if dist <= speed {
                pos.0 = target.0;
                pos.1 = target.1;
                continue;
            }
            
            //TODO: finish this
            pos.0 = (pos.0 as i64 + step_x as i64) as u32;
            pos.1 = (pos.1 as i64 + step_y as i64) as u32;
        }
    }
}