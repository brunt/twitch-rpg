use crate::ecs::components::class::CharacterClass;
use crate::ecs::components::{
    Enemy, Health, HealthComponent, Level, Money, Name, Player, Position, Projectile,
};
use crate::ecs::resources::{Adventure, CountdownTimer, GameState, ShopInventory};
use common::{EntityPosition, Form, GameSnapShot, ItemQuality, PlayerSnapshot, ShopItem};
use specs::{Join, ReadExpect, ReadStorage, System};
use std::time::Duration;
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
        ReadStorage<'a, Player>,
        ReadStorage<'a, Enemy>,
        ReadExpect<'a, GameState>,
        ReadExpect<'a, Option<CountdownTimer>>,
        ReadExpect<'a, ShopInventory>,
        ReadExpect<'a, Option<Adventure>>,
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
            players,
            enemies,
            game_state,
            countdown,
            shop_inventory,
            adventure,
        ): Self::SystemData,
    ) {
        let state = &*game_state;
        match state {
            GameState::InTown => {
                // TODO: builder method for this?
                let mut gs = GameSnapShot {
                    party: Vec::new(),
                    party_position: None,
                    floor_positions: None,
                    floor: None,
                    shop_items: Some(shop_inventory.items.clone()),
                    ready_timer: countdown.clone().map(|c| c.to_serialized()),
                    difficulty: None,
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
            GameState::OnAdventure => {
                // for (pos, player, enemy) in (&positions, &players, &enemies).join() {
                //
                // }

                // let in_game_players = (&players, &positions, &levels, &character_classes).join().map(|(player, name, pos, level)| {
                //
                // });

                let positions: Vec<EntityPosition> = (&positions, &character_classes, &levels)
                    .join()
                    .map(|(pos, class, lvl)| {
                        let pos = pos.clone();
                        let class = class.clone();
                        EntityPosition {
                            class: class.to_string(),
                            position: tatami_dungeon::Position { x: pos.x, y: pos.y },
                            level: lvl.0,
                        }
                    })
                    .chain((&positions, &names, &levels, &enemies).join().map(
                        |(pos, name, level, _)| EntityPosition {
                            class: name.0.clone(),
                            position: tatami_dungeon::Position { x: pos.x, y: pos.y },
                            level: level.0,
                        },
                    ))
                    .collect();

                let mut gs = GameSnapShot {
                    party_position: adventure
                        .clone()
                        .and_then(|ad| Some(ad.dungeon.player_position)),
                    party: Vec::new(),
                    floor_positions: Some(positions),
                    floor: adventure
                        .clone()
                        .map_or(None, |dungeon| Some(dungeon.get_floor_data())), //TODO: transition between dungeon floors
                    shop_items: None,
                    ready_timer: None,
                    difficulty: Some(1), //TODO: decide how to set dungeon difficulty
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
                        form: Form::Normal, // TODO: not always normal, read buffs
                    });
                }
                // for (pos, health, proj) in (&positions, &health, &projectiles).join() {
                //     // gs.party.append()
                // }
                _ = self.sender.send(gs);
            }
        }
    }
}
