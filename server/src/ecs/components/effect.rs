use common::Effect;
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimedEffect {
    pub effect: Effect,
    pub remaining_secs: f64,
}

impl PartialEq for TimedEffect {
    fn eq(&self, other: &Self) -> bool {
        self.effect == other.effect
    }
}

#[derive(Component, Debug, Default)]
pub struct ActiveEffects {
    pub effects: Vec<TimedEffect>,
}
