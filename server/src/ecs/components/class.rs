use crate::ecs::components::Component;
use crate::ecs::components::DenseVecStorage;
use common::PlayerClass;
use std::fmt::{Display, Formatter};

#[derive(Debug, Component, Clone)]
pub struct CharacterClass(pub PlayerClass);

