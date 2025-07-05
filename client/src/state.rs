use crate::sprites::SpriteRect;

//TODO: this belongs in game state struct
pub struct Player {
    pub(crate) name: String,
    pub health: u32,
    pub(crate) sprite: SpriteRect,
    pub gold: u32,
    pub weapon_slot: String,
    pub armor_slot: String,
    pub inventory: Vec<String>,
}