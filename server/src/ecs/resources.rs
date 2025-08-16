use common::{MenuItem, SerializedCountdownTimer, ShopItem};
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use tatami_dungeon::{
    Direction, Dungeon as TatamiDungeon, GenerateDungeonParams, Position, Room, Tile,
};

#[derive(Debug, Clone)]
pub enum GameState {
    InTown,
    OnAdventure,
    AfterDungeon,
}

#[derive(Clone, Debug)]
pub struct ExplorationTree {
    pub parent_links: HashMap<u32, u32>,
    pub root: u32,
}

impl ExplorationTree {
    pub fn new(root_id: u32) -> Self {
        Self {
            parent_links: HashMap::new(),
            root: root_id,
        }
    }

    pub fn add_child(&mut self, parent_id: u32, child_id: u32) {
        self.parent_links.insert(child_id, parent_id);
    }

    pub fn get_all_explored_ids(&self) -> HashSet<u32> {
        let mut explored: HashSet<u32> = self.parent_links.keys().cloned().collect();
        explored.insert(self.root);
        explored
    }

    pub fn contains(&self, room_id: &u32) -> bool {
        *room_id == self.root || self.parent_links.contains_key(room_id)
    }

    /// Finds the correct room to backtrack to for a depth-first search.
    /// It starts from the given room and traverses up its parents until it finds
    /// a room with an unexplored connection.
    pub fn find_next_room_for_dfs(
        &self,
        current_room_id: u32,
        floor: &tatami_dungeon::Floor,
    ) -> Option<u32> {
        // Start from the current room and backtrack up to the root.
        let mut backtrack_candidate = current_room_id;

        loop {
            // Check if the current candidate room has any unexplored children.
            if let Some(room) = floor.rooms.iter().find(|r| r.id == backtrack_candidate)
                && room.connections.iter().any(|conn| !self.contains(&conn.id))
            {
                // This room has unexplored paths, so this is our target.
                return Some(backtrack_candidate);
            }

            // If not, move to the parent.
            if let Some(&parent_id) = self.parent_links.get(&backtrack_candidate) {
                backtrack_candidate = parent_id;
            } else {
                // We've reached the root and there's no parent, so we're done.
                // This implies the entire reachable part of the dungeon is explored.
                return None;
            }
        }
    }
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
    pub difficulty: u32,

    /// room IDs of rooms that players have visited (cleared when entering new floor?)
    pub explored_rooms: ExplorationTree,
}

impl Adventure {
    pub fn generate_with_params(params: GenerateDungeonParams) -> Self {
        let dungeon = TatamiDungeon::generate_with_params(params);
        let starting_room_id = dungeon.starting_room_id;
        let explored_rooms = ExplorationTree::new(starting_room_id);

        Self {
            dungeon,
            current_floor_index: 0,
            current_room_index: starting_room_id,
            difficulty: 1, //TODO: decide how to set dungeon difficulty
            explored_rooms,
        }
    }

    pub fn get_current_floor(&self) -> &tatami_dungeon::Floor {
        &self.dungeon.floors[self.current_floor_index]
    }

    // get a room by id
    pub fn get_room_by_id(&self, room_id: u32) -> Option<&Room> {
        self.get_current_floor()
            .rooms
            .iter()
            .find(|room| room.id == room_id)
    }

    pub fn get_floor_data(&self) -> Vec<Vec<u8>> {
        let floor = &self.get_current_floor();

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
        let floor = &self.get_current_floor();
        let visible_room_ids = self.explored_rooms.get_all_explored_ids();

        let height = floor.tiles.len();
        let width = floor.tiles.first().map(|row| row.len()).unwrap_or(0);

        let mut visible = vec![vec![false; width]; height];

        use std::collections::HashSet;
        let mut trap_positions = HashSet::new();
        let mut teleporter_positions = HashSet::new();
        let mut stairs_positions = HashSet::new();

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
                for trap in &room.traps {
                    trap_positions.insert((trap.position.x as usize, trap.position.y as usize));
                }
                for tele in &room.teleporters {
                    teleporter_positions
                        .insert((tele.position.x as usize, tele.position.y as usize));
                }
                for stairs in &room.stairs {
                    stairs_positions
                        .insert((stairs.position.x as usize, stairs.position.y as usize));
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
                            if teleporter_positions.contains(&(x, y)) {
                                3
                            } else if trap_positions.contains(&(x, y)) {
                                4
                            } else if stairs_positions.contains(&(x, y)) {
                                5
                            } else {
                                match tile {
                                    Tile::Floor => 1,
                                    Tile::Wall => 2,
                                }
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
        self.get_current_floor()
            .rooms
            .iter()
            .flat_map(|room| room.enemies.iter().map(|enemy| enemy.position))
            .collect()
    }

    pub fn get_item_data(&self) -> Vec<Position> {
        self.get_current_floor()
            .rooms
            .iter()
            .flat_map(|room| room.items.iter().map(|item| item.position))
            .collect()
    }

    pub fn get_visible_item_data(&self) -> Vec<Position> {
        let floor = &self.get_current_floor();
        let explored = self.explored_rooms.get_all_explored_ids();
        floor
            .rooms
            .iter()
            .filter(|room| explored.contains(&room.id)) // Only explored rooms
            .flat_map(|room| room.items.iter().map(|enemy| enemy.position))
            .collect()
    }

    pub fn get_room_enemy_data(&self, room_id: u32) -> Vec<Position> {
        let floor = &self.get_current_floor();
        floor
            .rooms
            .iter()
            .filter(|room| room.id == room_id)
            .flat_map(|room| room.enemies.iter().map(|enemy| enemy.position))
            .collect()
    }

    // fn find_nearest_floor_spawn(start: Position, floor: &Floor) -> Option<Position> {
    pub(crate) fn find_nearest_floor_spawn(&self, start: &Position) -> Option<Position> {
        let floor = &self.get_current_floor();

        let (width, height) = (floor.tiles[0].len() as u32, floor.tiles.len() as u32);

        // First, try the given position
        if floor.tile_at(*start) == Tile::Floor {
            return Some(*start);
        }

        start
            .adjacent_8((width, height))
            .iter()
            .find(|adj| floor.tile_at(**adj) == Tile::Floor)
            .copied()
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

pub(crate) trait DirectionOffset {
    fn to_offset(&self) -> (i32, i32);
}

impl DirectionOffset for tatami_dungeon::Direction {
    fn to_offset(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Right => (-1, 0),
            Direction::Left => (1, 0),
        }
    }
}

impl Default for Adventure {
    fn default() -> Self {
        Self::generate_with_params(GenerateDungeonParams {
            max_stairs_per_floor: 1,
            min_stairs_per_floor: 1,

            ..GenerateDungeonParams::default()
        })
    }
}

#[derive(Default)]
pub struct ShopInventory {
    pub items: HashMap<MenuItem, ShopItem>,
}

#[derive(Default, Clone, Debug)]
pub struct DungeonLoot {
    pub items: u32,
}

/// ensure all players move to the same next room
pub struct GroupDestination {
    pub target_room_id: Option<u32>,
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
        Self::new(Duration::from_secs(5))
    }
}

#[derive(Debug, Clone, Default)]
pub struct DeltaTime(pub f64);
