use pathfinding::num_traits::SaturatingAdd;
use specs::prelude::*;
use crate::ecs::resources::DeltaTime;
use crate::ecs::components::effect::{ActiveEffects, TimedEffect};
use crate::ecs::components::form::FormComponent;
use common::{Effect, Form};
use crate::ecs::components::Stats;

pub struct EffectExpirationSystem;

impl<'a> System<'a> for EffectExpirationSystem {
    type SystemData = (
        WriteStorage<'a, ActiveEffects>,
        WriteStorage<'a, FormComponent>,
        WriteStorage<'a, Stats>,
        Read<'a, DeltaTime>,
        Entities<'a>,
    );

    fn run(&mut self, (mut active_effects, mut forms, mut stats, delta, entities): Self::SystemData) {
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
                        //TODO: system where stats change hp, etc
                        Effect::StatChange(stat_change) => {
                            if let Some(entity_stats) = stats.get_mut(entity) {
                                if let Some(delta_str) = stat_change.strength {
                                    entity_stats.strength = (entity_stats.strength as i32 - delta_str).max(0) as u32;
                                }
                                if let Some(delta_agi) = stat_change.agility {
                                    entity_stats.agility = (entity_stats.agility as i32 - delta_agi).max(0) as u32;
                                }
                                if let Some(delta_int) = stat_change.intelligence {
                                    entity_stats.intelligence = (entity_stats.intelligence as i32 - delta_int).max(0) as u32;
                                }
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
