use std::time::Duration;
use crate::ecs::components::class::CharacterClass;
use crate::ecs::components::{Health, HealthComponent, Level, Money, Name, Position, Projectile};
use crate::ecs::resources::{CountdownTimer, GameState, ShopInventory};
use common::{Form, GameSnapShot, ItemQuality, PlayerSnapshot, ShopItem};
use specs::{Join, ReadExpect, ReadStorage, System};
use tatami_dungeon::Dungeon;
use tokio::sync::broadcast::Sender;

pub struct Rendering {
    pub sender: Sender<GameSnapShot>,
}

impl<'a> System<'a> for Rendering {
    type SystemData = (
        ReadStorage<'a, Name>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, HealthComponent>,
        ReadStorage<'a, Projectile>,
        ReadStorage<'a, CharacterClass>,
        ReadStorage<'a, Level>,
        ReadStorage<'a, Money>,
        ReadExpect<'a, GameState>,
        ReadExpect<'a, Option<CountdownTimer>>,
        ReadExpect<'a, ShopInventory>,
        ReadExpect<'a, Dungeon>,
        // inventory
    );

    fn run(
        &mut self,
        (
            names,
            positions,
            health,
            projectiles,
            character_classes,
            levels,
            monies,
            game_state,
            countdown,
            shop_inventory,
            dungeon,
        ): Self::SystemData,
    ) {
        let state = &*game_state;
        match state {
            GameState::OutOfDungeon => {
                // TODO: builder method for this?
                let mut gs = GameSnapShot {
                    party: Vec::new(),
                    floor_positions: None,
                    floor: None,
                    shop_items: Some(shop_inventory.items.clone()),
                    ready_timer: countdown.clone().map(|c| c.to_serialized()),
                };

                for (name, health, character_class, level, money) in
                    (&names, &health, &character_classes, &levels, &monies).join()
                {
                    gs.party.push(PlayerSnapshot {
                        name: name.0.clone(), //TODO: not clone?
                        class: character_class.0.clone(),
                        health: health.0.clone(),
                        level: level.0.clone(),
                        gold: money.0,
                        form: Form::Normal,
                    });
                }
                _ = self.sender.send(gs);
            }
            GameState::InDungeon => {
                let mut gs = GameSnapShot {
                    party: Vec::new(),
                    floor_positions: Some(vec![]),
                    floor: dungeon.floors.get(0).cloned(), //TODO: transition between dungeon floors
                    shop_items: None,
                    ready_timer: None,
                };

                for (name, health, character_class, level, money) in
                    (&names, &health, &character_classes, &levels, &monies).join()
                {
                    gs.party.push(PlayerSnapshot {
                        name: name.0.clone(),
                        class: character_class.0.clone(),
                        health: health.0.clone(),
                        level: level.0.clone(),
                        gold: money.0,
                        form: Form::Normal // TODO: not always normal
                    });
                }
                for (pos, health, proj) in (&positions, &health, &projectiles).join() {
                    // gs.party.append()
                }
                _ = self.sender.send(gs);
            }
        }
    }
}
