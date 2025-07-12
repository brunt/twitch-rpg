use crate::ecs::components::class::CharacterClass;
use crate::ecs::components::{Health, HealthComponent, Level, Money, Name, Position, Projectile};
use crate::ecs::resources::{CountdownTimer, GameState};
use common::{GameSnapShot, PlayerSnapshot, ShopItem};
use specs::{Join, ReadExpect, ReadStorage, System};
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
                    shop_items: Some(vec![ShopItem {
                        name: "Name".to_string(),
                        description: "desc".to_string(),
                        price: 10,
                        sprite: "red_potion".to_string(),
                    }]),
                    ready_timer: countdown.clone().map(|c| c.to_serialized()),
                };

                for (name, health, character_class, level, money) in
                    (&names, &health, &character_classes, &levels, &monies).join()
                {
                    gs.party.push(PlayerSnapshot {
                        name: name.0.clone(), //TODO: not clone?
                        class: character_class.get_player_class(),
                        health: health.0.clone(),
                        level: level.0.clone(),
                        gold: money.0,
                        sprite_name: "wizard1".to_string(), //TODO: you know
                    });
                }
                _ = self.sender.send(gs);
            }
            GameState::InDungeon => {
                let mut gs = GameSnapShot {
                    party: Vec::new(),
                    floor_positions: Some(vec![]),
                    floor: None,
                    shop_items: None,
                    ready_timer: None,
                };

                for (name, health, character_class, level, money) in
                    (&names, &health, &character_classes, &levels, &monies).join()
                {
                    gs.party.push(PlayerSnapshot {
                        name: name.0.clone(), //TODO: not clone?
                        class: character_class.get_player_class(),
                        health: health.0.clone(),
                        level: level.0.clone(),
                        gold: money.0,
                        sprite_name: "wizard1".to_string(), //TODO: you know
                    });
                }
                for (pos, health, proj) in (&positions, &health, &projectiles).join() {
                    // gs.party.append()
                }
                dbg!("sending in dungeon");
                _ = self.sender.send(gs);
            }
        }
    }
}
