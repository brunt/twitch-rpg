use crate::ecs::components::combat::AttackTimer;
use crate::ecs::components::movement::CanMove;
use crate::ecs::resources::DeltaTime;
use specs::{Entities, Join, Read, System, WriteStorage};

pub struct AttackCooldownSystem;
impl<'a> System<'a> for AttackCooldownSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, AttackTimer>,
        WriteStorage<'a, CanMove>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, (entities, mut timers, mut can_moves, delta): Self::SystemData) {
        let mut to_reset = vec![];

        for (entity, timer) in (&entities, &mut timers).join() {
            if timer.remaining <= delta.0 {
                to_reset.push(entity);
            } else {
                timer.remaining -= delta.0;
            }
        }

        for entity in to_reset {
            timers.remove(entity);
            can_moves.insert(entity, CanMove).ok();
        }
    }
}
