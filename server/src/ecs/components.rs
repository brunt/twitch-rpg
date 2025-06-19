use specs::prelude::*;
use specs_derive::Component;

//Entity's coordinates in the world
#[derive(Debug, Component)]
pub(crate) struct Position(pub(crate) u32, pub(crate) u32);

#[derive(Debug, Component)]
pub(crate) struct MovementSpeed(u32);

#[derive(Debug, Component)]
pub(crate) struct TargetPosition(u32, u32);

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
    Alive {
        hp: u32,
    },
    Dead
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
struct EquipmentSlot{
    weapon: Option<String>,
}	//List of items held



// #[derive(Debug)]
// struct Stats();	//RPG stats (strength, agility, etc.)
#[derive(Debug)]
struct Experience(u32);	//XP and level

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
