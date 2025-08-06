use crate::ecs::components::Component;
use crate::ecs::components::DenseVecStorage;
use crate::ecs::components::NullStorage;
use common::{EquipmentSlot, Item};
use std::collections::HashMap;

#[derive(Component, Debug, Default)]
pub struct Equipment {
    pub slots: HashMap<EquipmentSlot, Item>,
}

#[derive(Component)]
#[storage(NullStorage)]
pub struct Consumable;
