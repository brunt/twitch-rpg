use crate::commands::{MenuItem, Player};
use serde::Serialize;
use specs::{Entity, World};
use std::collections::HashMap;
use tatami_dungeon::{Dungeon, GenerateDungeonParams, Item};

#[derive(Debug, Clone)]
pub enum GameState {
    OutOfDungeon,
    InDungeon,
}

//TODO: shouldn't need to show this at all
// impl Serialize for GameState {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         match self {
//             GameState::OutOfDungeon => serializer.serialize_bool(false),
//             GameState::InDungeon => serializer.serialize_bool(true),
//         }
//     }
// }

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

/// Struct that holds game data specifically for the client to render

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
