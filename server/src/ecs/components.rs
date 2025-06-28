use specs::prelude::*;
use specs_derive::Component;
use std::cmp::Ordering;

//Entity's coordinates in the world
#[derive(Debug, Component, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Component, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct MovementSpeed(pub i32);

impl PartialEq<i32> for MovementSpeed {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}
impl PartialEq<MovementSpeed> for i32 {
    fn eq(&self, other: &MovementSpeed) -> bool {
        *self == other.0
    }
}
impl PartialOrd<i32> for MovementSpeed {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialOrd<MovementSpeed> for i32 {
    fn partial_cmp(&self, other: &MovementSpeed) -> Option<Ordering> {
        self.partial_cmp(&other.0)
    }
}

#[derive(Debug, Component)]
pub struct TargetPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Component)]
pub struct Renderable {
    /// name of texture
    pub texture_name: String,

    pub i_w: u32,
    pub i_h: u32,
    pub o_w: u32,
    pub o_h: u32,

    pub frame: u32,
    pub total_frames: u32,
}

#[derive(Debug, Component)]
enum Health {
    Alive { hp: u32 },
    Dead,
}

enum Equipment {
    Weapon,
    Armor(ArmorType),
}

enum ArmorType {
    Head,
    Chest,
    Jewelry,
    Hands,
    Feet,
    Legs,
}

#[derive(Debug, Component)]
struct EquipmentSlot {
    weapon: Option<String>,
} //List of items held

// #[derive(Debug)]
// struct Stats();	//RPG stats (strength, agility, etc.)
#[derive(Debug)]
struct Experience(u32); //XP and level

#[derive(Debug, Component)]
struct Level(u32);

#[derive(Component, Debug)]
enum Faction {
    Player,
    Enemy,
    Neutral,
}

#[derive(Component)]
#[storage(NullStorage)]
struct Immobile;
