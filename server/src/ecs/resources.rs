use common::{MenuItem, SerializedCountdownTimer, ShopItem};
use specs::{Entity, World};
use std::collections::{HashMap, HashSet};
use std::time::Duration;

use tatami_dungeon::{Dungeon as TatamiDungeon, Floor, GenerateDungeonParams, Item, Position, Room, Tile};

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

    /// room IDs of rooms that players have visited (cleared when entering new floor?)
    pub explored_rooms: HashSet<u32>,
}

impl Adventure {
    pub fn generate_with_params(params: GenerateDungeonParams) -> Self {
        let mut dungeon = TatamiDungeon::generate_with_params(params);
        let mut explored_rooms = HashSet::new();
        explored_rooms.insert(dungeon.starting_room_id);


        Self {
            dungeon,
            current_floor_index: 0,
            difficulty: 0,
            explored_rooms,
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
                        Tile::Floor => 1,
                        Tile::Wall => 2,
                    })
                    .collect()
            })
            .collect()
    }

    pub fn filter_visible_rooms(&self) -> Vec<Vec<u8>> {
        let floor = &self.dungeon.floors[self.current_floor_index];
        let visible_room_ids = &self.explored_rooms;

        let height = floor.tiles.len();
        let width = floor.tiles.get(0).map(|row| row.len()).unwrap_or(0);

        // Build a boolean mask of which tiles are visible
        let mut visible = vec![vec![false; width]; height];

        for room in &floor.rooms {
            if visible_room_ids.contains(&room.id) {
                let start_x = room.position.x as usize;
                let start_y = room.position.y as usize;
                let end_x = (start_x + room.width as usize).min(width);
                let end_y = (start_y + room.height as usize).min(height);
                for y in start_y..end_y {
                    for x in start_x..end_x {
                        visible[y][x] = true;
                    }
                }
            }
        }

        floor
            .tiles
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, tile)| {
                        if visible[y][x] {
                            match tile {
                                Tile::Floor => 1,
                                Tile::Wall => 2,
                            }
                        } else {
                            0
                        }
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

    // fn find_nearest_floor_spawn(start: Position, floor: &Floor) -> Option<Position> {
    pub(crate) fn find_nearest_floor_spawn(&self) -> Option<Position> {
        let start = self.dungeon.player_position;
        let floor = &self.dungeon.floors[self.current_floor_index];

        let (width, height) = (
            floor.tiles[0].len() as u32,
            floor.tiles.len() as u32,
        );

        // First, try the given position
        if floor.tiles[start.y as usize][start.x as usize] == Tile::Floor {
            return Some(start);
        }

        // Try all adjacent-8 positions
        for adj in start.adjacent_8((width, height)) {
            if floor.tiles[adj.y as usize][adj.x as usize] == Tile::Floor {
                return Some(adj);
            }
        }
        None
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
