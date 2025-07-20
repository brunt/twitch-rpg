use common::{MenuItem, SerializedCountdownTimer, ShopItem};
use std::collections::{HashMap, HashSet};
use std::time::Duration;

use tatami_dungeon::{Dungeon as TatamiDungeon, GenerateDungeonParams, Position, Room, Tile};

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

    /// index which tells what room the players are in
    pub current_room_index: u32,

    /// difficulty determines enemies, tileset, loot quality, enemy density
    pub difficulty: usize,

    /// room IDs of rooms that players have visited (cleared when entering new floor?)
    pub explored_rooms: HashSet<u32>,
}

impl Adventure {
    pub fn generate_with_params(params: GenerateDungeonParams) -> Self {
        let mut dungeon = TatamiDungeon::generate_with_params(params);
        let mut explored_rooms = HashSet::new();
        let starting_room_id = dungeon.starting_room_id;
        explored_rooms.insert(starting_room_id);

        Self {
            dungeon,
            current_floor_index: 0,
            current_room_index: starting_room_id,
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
                        visible[x][y] = true;
                    }
                }
            }
        }

        floor
            .tiles
            .iter()
            .enumerate()
            .map(|(x, col)| {
                col.iter()
                    .enumerate()
                    .map(|(y, tile)| {
                        if visible[x][y] {
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

    pub fn get_item_data(&self) -> Vec<Position> {
        let data = self.dungeon.floors[self.current_floor_index]
            .rooms
            .iter()
            .flat_map(|room| room.items.iter().map(|item| item.position))
            .collect();
        data
    }

    pub fn get_visible_enemy_data(&self) -> Vec<Position> {
        let floor = &self.dungeon.floors[self.current_floor_index];
        let explored = &self.explored_rooms;
        floor
            .rooms
            .iter()
            .filter(|room| explored.contains(&room.id)) // Only explored rooms
            .flat_map(|room| room.enemies.iter().map(|enemy| enemy.position))
            .collect()
    }

    // fn find_nearest_floor_spawn(start: Position, floor: &Floor) -> Option<Position> {
    pub(crate) fn find_nearest_floor_spawn(&self, start: &Position) -> Option<Position> {
        let floor = &self.dungeon.floors[self.current_floor_index];

        let (width, height) = (floor.tiles[0].len() as u32, floor.tiles.len() as u32);

        // First, try the given position
        if floor.tile_at(*start) == Tile::Floor {
            return Some(*start);
        }

        // Try all adjacent-8 positions
        for adj in start.adjacent_8((width, height)) {
            if floor.tile_at(adj) == Tile::Floor {
                return Some(adj);
            }
        }
        None
    }
}

pub(crate) trait RoomCheck {
    fn contains(&self, p: &Position) -> bool;
}

impl RoomCheck for Room {
    fn contains(&self, p: &Position) -> bool {
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
        Self::new(Duration::from_secs(60))
    }
}

#[derive(Debug, Clone, Default)]
pub struct DeltaTime(pub f64);
