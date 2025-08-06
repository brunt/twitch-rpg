use specs::prelude::*;
use crate::ecs::resources::DeltaTime;
use crate::ecs::components::effect::{ActiveEffects, TimedEffect};
use crate::ecs::components::form::FormComponent;
use common::{Effect, Form};

pub struct EffectExpirationSystem;

impl<'a> System<'a> for EffectExpirationSystem {
    type SystemData = (
        WriteStorage<'a, ActiveEffects>,
        WriteStorage<'a, FormComponent>,
        Read<'a, DeltaTime>,
        Entities<'a>,
    );

    fn run(&mut self, (mut active_effects, mut forms, delta, entities): Self::SystemData) {
        let delta_secs = delta.0;

        for (entity, active) in (&entities, &mut active_effects).join() {
            let mut retained_effects = Vec::new();

            for mut timed in active.effects.drain(..) {
                // Update duration
                if timed.remaining_secs > delta_secs {
                    timed.remaining_secs -= delta_secs;
                    retained_effects.push(timed);
                } else {
                    // Expired — revert the effect
                    match timed.effect {
                        Effect::Transform(_) => {
                            if let Some(form) = forms.get_mut(entity) {
                                form.0 = Form::Normal;
                            }
                        }
                        _ => {}
                    }
                }
            }

            active.effects = retained_effects;
        }
    }
}
