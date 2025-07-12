use crate::ecs::components::Component;
use crate::ecs::components::DenseVecStorage;
use common::PlayerClass;
use std::fmt::{Display, Formatter};

#[derive(Debug, Component, Clone)]
pub enum CharacterClass {
    PlayerClass(PlayerClass),
    Beast,
    Dragon,
    Elemental,
}

impl CharacterClass {
    pub fn is_player(&self) -> bool {
        match self {
            Self::PlayerClass(_) => true,
            _ => false,
        }
    }

    pub fn get_player_class(&self) -> Option<PlayerClass> {
        match self {
            Self::PlayerClass(pc) => Some(pc.clone()),
            _ => None,
        }
    }
}

impl Display for CharacterClass {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PlayerClass(pc) => write!(f, "{}", pc),
            Self::Beast => write!(f, "Beast"),
            Self::Dragon => write!(f, "Dragon"),
            Self::Elemental => write!(f, "Elemental"),
        }
    }
}
