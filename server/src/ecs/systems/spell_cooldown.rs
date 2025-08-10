use crate::ecs::components::movement::CanMove;
use crate::ecs::components::spells::SpellTimer;
use crate::ecs::resources::DeltaTime;
use specs::prelude::*;

pub struct SpellCooldownSystem;

impl<'a> System<'a> for SpellCooldownSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, DeltaTime>,
        WriteStorage<'a, SpellTimer>,
        WriteStorage<'a, CanMove>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, delta_time, mut spell_timers, mut can_move) = data;

        let mut expired_timers = Vec::new();

        // TODO: idiomatic
        for (entity, timer) in (&entities, &mut spell_timers).join() {
            timer.remaining -= delta_time.0;

            if timer.remaining <= 0.0 {
                expired_timers.push(entity);
            }
        }

        for entity in expired_timers {
            spell_timers.remove(entity);
            can_move
                .insert(entity, CanMove)
                .expect("Failed to insert CanMove component after spell cooldown");
        }
    }
}
