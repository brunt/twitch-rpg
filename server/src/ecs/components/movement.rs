use crate::ecs::components;
use crate::ecs::components::Component;
use crate::ecs::components::DenseVecStorage;
use crate::ecs::components::inventory::Equipment;
use common::{DefenseModifiers, OtherModifiers};
use std::cmp::Ordering;

// Entity's coordinates in the world
#[derive(Component, Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn distance_to(&self, other: &Position) -> u32 {
        self.x.abs_diff(other.x).max(self.y.abs_diff(other.y))
    }
}

impl From<&tatami_dungeon::Position> for Position {
    fn from(p: &tatami_dungeon::Position) -> Self {
        Position { x: p.x, y: p.y }
    }
}

impl From<&Position> for tatami_dungeon::Position {
    fn from(p: &Position) -> Self {
        tatami_dungeon::Position { x: p.x, y: p.y }
    }
}

impl From<&TargetPosition> for tatami_dungeon::Position {
    fn from(p: &TargetPosition) -> Self {
        tatami_dungeon::Position { x: p.x, y: p.y }
    }
}

impl From<&tatami_dungeon::Position> for TargetPosition {
    fn from(p: &tatami_dungeon::Position) -> Self {
        TargetPosition { x: p.x, y: p.y }
    }
}

#[derive(Debug, Component, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct MovementSpeed(pub u32);

impl MovementSpeed {
    //TODO: from buffs as well
    pub fn from_items(equipment: &Equipment) -> Self {
        let other_mods = equipment.slots.iter().map(|(_slot, item)| item).fold(
            OtherModifiers {
                movement_speed_increase: 0,
            },
            |mut m, item| {
                if let Some(modifiers) = &item.stats.other_modifiers {
                    m.movement_speed_increase += modifiers.movement_speed_increase;
                }
                m
            },
        );

        Self(other_mods.movement_speed_increase.max(1))
    }
}

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

impl From<&Position> for TargetPosition {
    fn from(p: &Position) -> Self {
        TargetPosition { x: p.x, y: p.y }
    }
}

#[derive(Component)]
pub struct DesiredTargetPosition {
    pub x: u32,
    pub y: u32,
}

impl From<&DesiredTargetPosition> for TargetPosition {
    fn from(p: &DesiredTargetPosition) -> Self {
        TargetPosition { x: p.x, y: p.y }
    }
}

impl From<&tatami_dungeon::Position> for DesiredTargetPosition {
    fn from(p: &tatami_dungeon::Position) -> Self {
        DesiredTargetPosition { x: p.x, y: p.y }
    }
}

#[derive(Debug, Component, Clone)]
pub struct Path {
    pub steps: Vec<(u32, u32)>, // next steps, from start to end
}

#[derive(Debug, Component, Clone)]
pub struct Wander;
