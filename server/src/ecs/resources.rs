use crate::commands::{Item, MenuItem};
use serde::Serialize;
use specs::{Entity, World};
use std::collections::HashMap;
use tatami_dungeon::{Dungeon, GenerateDungeonParams};

#[derive(Debug, Clone)]
pub enum GameState {
    OutOfDungeon,
    InDungeon,
}

pub trait DungeonExt {
    fn generate_dungeon(&mut self, params: GenerateDungeonParams);
}

impl DungeonExt for World {
    fn generate_dungeon(&mut self, params: GenerateDungeonParams) {
        let dungeon = Dungeon::generate_with_params(params);
        self.insert(dungeon);
        self.insert(GameState::OutOfDungeon);
    }
}

// Struct that holds game data specifically for the client to render
#[derive(Clone, Serialize)]
pub struct GameSnapShot {}

// queue of players that can shop before joining dungeon
#[derive(Default)]
pub struct TownPlayers {
    pub players: HashMap<String, Entity>,
}

pub struct ShopInventory {
    pub items: HashMap<MenuItem, (Item, u32)>,
}

// impl ShopInventory {
//     pub fn new() -> Self {
//         let mut shop = Self::default();
//
//         // Add default shop items
//         // For example:
//         // shop.items.insert(MenuItem(1), (Item::Consumable(Consumable::Potion("Health".to_string())), 10));
//
//         shop
//     }
// }
