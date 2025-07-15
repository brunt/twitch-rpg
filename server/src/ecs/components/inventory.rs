use crate::ecs::components::Component;
use crate::ecs::components::DenseVecStorage;
use common::{EquipmentSlot, EquippedItem};
use specs::Entity;
use std::collections::HashMap;

#[derive(Component, Debug, Default)]
pub struct Equipment {
    pub slots: HashMap<EquipmentSlot, EquippedItem>,
}
