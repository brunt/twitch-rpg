use std::collections::HashSet;
use crate::ecs::components::class::CharacterClass;
use crate::ecs::components::movement::TargetPosition;
use crate::ecs::components::{
    DungeonItem, Enemy, Health, HealthComponent, Level, Money, Name, Player, Position, Projectile,
};
use crate::ecs::resources::{Adventure, CountdownTimer, GameState, ShopInventory};
use common::{EntityPosition, Form, GameSnapShot, ItemQuality, PlayerSnapshot, ShopItem};
use specs::{Join, LendJoin, ReadExpect, ReadStorage, System};
use tokio::sync::broadcast::Sender;


/// This system generates a struct that will get serialized to JSON and sent to the frontend.
/// Information from it will be used to draw to the canvas
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
        ReadStorage<'a, DungeonItem>,
        ReadStorage<'a, TargetPosition>,
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
            dungeon_items,
            target_positions,
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
                let positions: Vec<EntityPosition> = (
                    &positions,
                    &character_classes,
                    &levels,
                    target_positions.maybe(),
                )
                    .join()
                    .map(|(pos, class, lvl, target_pos_maybe)| {
                        let pos = pos.clone();
                        let class = class.clone();
                        EntityPosition {
                            class: class.to_string(),
                            position: tatami_dungeon::Position { x: pos.x, y: pos.y },
                            level: lvl.0,
                            target_position: if let Some(target_pos) = target_pos_maybe {
                                Some(tatami_dungeon::Position::from(target_pos))
                            } else {
                                None
                            },
                        }
                    })
                    .chain(
                        (
                            &positions,
                            &names,
                            &levels,
                            &enemies,
                            target_positions.maybe(),
                        )
                            .join()
                            .map(
                                |(pos, name, level, _, target_pos_maybe)| EntityPosition {
                                    class: name.0.clone(),
                                    position: tatami_dungeon::Position::from(pos),
                                    level: level.0,
                                    target_position: if let Some(target_pos) = target_pos_maybe {
                                        Some(tatami_dungeon::Position::from(target_pos))
                                    } else {
                                        None
                                    },
                                },
                            ),
                    )
                    .chain(
                        (&positions, &names, &dungeon_items)
                            .join()
                            .map(|(pos, name, item)| EntityPosition {
                                class: "Item".to_string(),
                                position: tatami_dungeon::Position::from(pos),
                                level: 0,
                                target_position: None,
                            }),
                    )
                    .collect();

                let mut gs = GameSnapShot {
                    party_position: adventure
                        .clone()
                        .and_then(|ad| Some(ad.dungeon.player_position)),
                    party: Vec::new(),
                    floor_positions: Some(positions),
                    floor: adventure
                        .clone()
                        .map_or(None, |dungeon| Some(dungeon.filter_visible_rooms())), //TODO: transition between dungeon floors
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
