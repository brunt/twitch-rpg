use crate::ecs::components::Component;
use crate::ecs::components::DenseVecStorage;
use common::Form;

#[derive(Component)]
pub struct FormComponent(pub Form);