use common::{MenuItem, SerializedCountdownTimer, ShopItem};
use specs::{Entity, World};
use std::collections::HashMap;
use std::time::Duration;
use tatami_dungeon::{Dungeon as TatamiDungeon, GenerateDungeonParams, Item, Tile};

#[derive(Debug, Clone)]
pub enum GameState {
    InTown,
    OnAdventure,
}

#[derive(Clone)]
pub struct Adventure {
    pub dungeon: TatamiDungeon,

    /// index which tells the frontend which floor to render
    pub current_floor_index: usize,

    /// difficulty determines enemies, tileset, loot quality, enemy density
    pub difficulty: usize,
}

impl Adventure {
    pub fn generate_with_params(params: GenerateDungeonParams) -> Self {
        let mut dungeon = TatamiDungeon::generate_with_params(params);
        for floor in &dungeon.floors {
            // for (x, col) in floor.tiles.iter().enumerate() {
            //     for (y, tile) in col.iter().enumerate() {
            //         match tile {
            //             Tile::Floor => // Draw floor tile at (x, y)
            //             Tile::Wall => // Draw wall tile at (x, y)
            //         }
            //     }
            // }

            // for room in &floor.rooms {
            //     for item in &room.items {
            //         match item.rarity {
            //             //     1..=20 => // Spawn common item
            //             //         21..=40 => // Spawn uncommon item
            //             //     41..=60 => // Spawn rare item
            //             //         61..=80 => // Spawn epic item
            //             //     81..=100 => // Spawn legendary item
            //             // _ => unreachable!(),
            //         }
            //     }
            // 
            //     for enemy in &room.enemies {
            //         match enemy.difficulty {
            //             // 1..=20 => // Spawn common enemy
            //             //     21..=40 => // Spawn uncommon enemy
            //             // 41..=60 => // Spawn rare enemy
            //             //     61..=80 => // Spawn epic enemy
            //             // 81..=100 => // Spawn legendary enemy
            //             _ => unreachable!(),
            //         }
            //     }
            // 
            //     for trap in &room.traps {
            //         match trap.difficulty {
            //             // 1..=20 => // Spawn common trap
            //             //     21..=40 => // Spawn uncommon trap
            //             // 41..=60 => // Spawn rare trap
            //             //     61..=80 => // Spawn epic trap
            //             // 81..=100 => // Spawn legendary trap
            //             _ => unreachable!(),
            //         }
            //     }
            // }
        }

        Self {
            dungeon,
            current_floor_index: 0,
            difficulty: 0,
        }
    }
    
    pub fn get_floor_data(&self) -> Vec<Vec<u8>> {
        let floor = &self.dungeon.floors[self.current_floor_index];

        floor.tiles.iter().map(|row| {
            row.iter().map(|tile| match tile {
                Tile::Floor => 0,
                Tile::Wall => 1,
            }).collect()
        }).collect()
    }
}

impl Default for Adventure {
    fn default() -> Self {
        Self::generate_with_params(GenerateDungeonParams::default())
    }
}

#[derive(Default)]
pub struct ShopInventory {
    pub items: HashMap<MenuItem, ShopItem>,
}

#[derive(Clone)]
pub struct CountdownTimer {
    pub remaining: Duration,
}

impl CountdownTimer {
    pub fn new(remaining: Duration) -> Self {
        Self { remaining }
    }

    pub fn to_serialized(&self) -> SerializedCountdownTimer {
        SerializedCountdownTimer {
            remaining: self.remaining.as_secs(),
        }
    }
}

impl Default for CountdownTimer {
    fn default() -> Self {
        Self::new(Duration::from_secs(3))
    }
}

#[derive(Debug, Clone, Default)]
pub struct DeltaTime(pub f64);
