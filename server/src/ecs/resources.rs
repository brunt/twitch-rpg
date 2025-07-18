use common::{MenuItem, SerializedCountdownTimer, ShopItem};
use specs::{Entity, World};
use std::collections::HashMap;
use std::time::Duration;

use tatami_dungeon::{Dungeon as TatamiDungeon, GenerateDungeonParams, Item, Position, Room, Tile};

#[derive(Debug, Clone)]
pub enum GameState {
    InTown,
    OnAdventure,
}

#[derive(Clone)]
pub struct Adventure {
    /// PlayerPosition in this struct is only used for initial placement in ECS
    pub dungeon: TatamiDungeon,

    /// index which tells the frontend which floor to render
    pub current_floor_index: usize,

    /// difficulty determines enemies, tileset, loot quality, enemy density
    pub difficulty: usize,
}

impl Adventure {
    pub fn generate_with_params(params: GenerateDungeonParams) -> Self {
        let mut dungeon = TatamiDungeon::generate_with_params(params);

        Self {
            dungeon,
            current_floor_index: 0,
            difficulty: 0,
        }
    }

    pub fn get_floor_data(&self) -> Vec<Vec<u8>> {
        let floor = &self.dungeon.floors[self.current_floor_index];

        floor
            .tiles
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tile| match tile {
                        Tile::Floor => 0,
                        Tile::Wall => 1,
                    })
                    .collect()
            })
            .collect()
    }

    pub fn get_enemy_data(&self) -> Vec<Position> {
        self.dungeon.floors[self.current_floor_index]
            .rooms
            .iter()
            .flat_map(|room| room.enemies.iter().map(|enemy| enemy.position))
            .collect()
    }
}

pub(crate) trait RoomCheck {
    fn contains(&self, p: Position) -> bool;
}

impl RoomCheck for Room {
    fn contains(&self, p: Position) -> bool {
        let rx = self.position.x;
        let ry = self.position.y;
        let rw = self.width;
        let rh = self.height;

        p.x >= rx && p.x < rx + rw && p.y >= ry && p.y < ry + rh
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
        Self::new(Duration::from_secs(2))
    }
}

#[derive(Debug, Clone, Default)]
pub struct DeltaTime(pub f64);
