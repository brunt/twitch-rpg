use std::collections::HashMap;
use specs::Entity;
use crate::ecs::components::Component;
use crate::ecs::components::DenseVecStorage;
use common::{EquipmentSlot, EquippedItem};

#[derive(Component, Debug, Default)]
pub struct Equipment {
    pub slots: HashMap<EquipmentSlot, EquippedItem>
}