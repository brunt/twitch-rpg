use rand::{Rng, SeedableRng};
use specs::{Join, ReadStorage, System, WriteStorage};
use crate::ecs::components::{Position, TargetPosition};

pub struct RandomWander;

impl<'a> System<'a> for RandomWander {
    type SystemData = (
        WriteStorage<'a, TargetPosition>,
        ReadStorage<'a, Position>,
    );
    
    fn run(&mut self, (mut targets, positions): Self::SystemData) {
        let mut rng = rand::rngs::SmallRng::from_os_rng();
        
        for (target, pos) in (&mut targets, &positions).join() {
            if pos.x == target.x && pos.y == target.y {
                target.x = rng.random_range(0..10);
                target.y = rng.random_range(0..10);
                
                dbg!(target);
            }
        }
    }
}