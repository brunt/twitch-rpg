use serde::{Deserialize, Serialize};
use common::Effect;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimedEffect {
    pub effect: Effect,
    pub remaining_secs: f64,
}

#[derive(Component, Debug, Default)]
pub struct ActiveEffects {
    pub effects: Vec<TimedEffect>,
}