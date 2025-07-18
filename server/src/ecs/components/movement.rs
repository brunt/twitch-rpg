use crate::ecs::components::Component;
use crate::ecs::components::DenseVecStorage;
use std::cmp::Ordering;

// Entity's coordinates in the world
#[derive(Component, Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Component, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct MovementSpeed(pub u32);

impl PartialEq<u32> for MovementSpeed {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}
impl PartialEq<MovementSpeed> for u32 {
    fn eq(&self, other: &MovementSpeed) -> bool {
        *self == other.0
    }
}
impl PartialOrd<u32> for MovementSpeed {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<MovementSpeed> for u32 {
    fn partial_cmp(&self, other: &MovementSpeed) -> Option<Ordering> {
        self.partial_cmp(&other.0)
    }
}

#[derive(Debug, Default, Component, Clone)]
pub struct TargetPosition {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Component, Clone)]
pub struct Path {
    pub steps: Vec<(u32, u32)>, // next steps, from start to end
}

#[derive(Debug, Component, Clone)]
pub struct Wander;
