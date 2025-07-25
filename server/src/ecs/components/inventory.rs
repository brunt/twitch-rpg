use crate::ecs::components::Component;
use crate::ecs::components::DenseVecStorage;
use common::{AttackModifiers, EquipmentSlot, EquippedItem};
use std::collections::HashMap;

#[derive(Component, Debug, Default)]
pub struct Equipment {
    pub slots: HashMap<EquipmentSlot, EquippedItem>,
}
