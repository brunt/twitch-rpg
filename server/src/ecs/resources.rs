use common::{MenuItem, SerializedCountdownTimer, ShopItem};
use specs::{Entity, World};
use std::collections::HashMap;
use std::time::Duration;
use tatami_dungeon::{Dungeon, GenerateDungeonParams, Item};

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
