use crate::ecs::components::class::CharacterClass;
use crate::ecs::components::movement::TargetPosition;
use crate::ecs::components::{DungeonItem, Enemy, Level, Money, Name, Opened, Player, Position, Projectile};
use crate::ecs::resources::{Adventure, CountdownTimer, GameState, ShopInventory};
use common::{EntityPosition, Form, GameSnapShot, ItemQuality, PlayerSnapshot, ShopItem};
use specs::{Join, LendJoin, ReadExpect, ReadStorage, System};
use tokio::sync::broadcast::Sender;
use crate::ecs::components::combat::HealthComponent;

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
        ReadStorage<'a, Opened>,
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
            opened,
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
                    camera_position: None,
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
                let entity_positions: Vec<EntityPosition> = (
                    &positions,
                    &character_classes,
                    &levels,
                    &health,
                    target_positions.maybe(),
                )
                    .join()
                    .map(|(pos, class, lvl, health, target_pos_maybe)| {
                        let pos = pos.clone();
                        let class = class.clone();
                        EntityPosition {
                            entity_type: class.to_string(),
                            position: tatami_dungeon::Position { x: pos.x, y: pos.y },
                            level: lvl.0,
                            target_position: if let Some(target_pos) = target_pos_maybe {
                                Some(tatami_dungeon::Position::from(target_pos))
                            } else {
                                None
                            },
                            health: Some(health.0.clone()),
                        }
                    })
                    .chain(
                        (
                            &positions,
                            &names,
                            &levels,
                            &enemies,
                            &health,
                            target_positions.maybe(),
                        )
                            .join()
                            .map(
                                |(pos, name, level, _, health, target_pos_maybe)| EntityPosition {
                                    entity_type: name.0.clone(),
                                    position: tatami_dungeon::Position::from(pos),
                                    level: level.0,
                                    target_position: if let Some(target_pos) = target_pos_maybe {
                                        Some(tatami_dungeon::Position::from(target_pos))
                                    } else {
                                        None
                                    },
                                    health: Some(health.0.clone()),
                                },
                            ),
                    )
                    .chain(
                        (&positions, &dungeon_items, opened.maybe())
                            .join()
                            .map(|(pos, item, opened_maybe)| EntityPosition {
                                entity_type: if opened_maybe.is_some() {"Opened".to_string() } else { "Item".to_string()},
                                position: tatami_dungeon::Position::from(pos),
                                level: 0,
                                target_position: None,
                                health: None,
                            }),
                    )
                    .collect();


                let mut min_x = u32::MAX;
                let mut max_x = 0;
                let mut min_y = u32::MAX;
                let mut max_y = 0;

                for (pos, _) in (&positions, &players).join() {
                    min_x = min_x.min(pos.x);
                    max_x = max_x.max(pos.x);
                    min_y = min_y.min(pos.y);
                    max_y = max_y.max(pos.y);
                }
                
                let mut gs = GameSnapShot {
                    camera_position: compute_camera_position(min_x, max_x, min_y, max_y),
                    party: Vec::new(),
                    floor_positions: Some(entity_positions),
                    floor: adventure
                        .clone()
                        .map_or(None, |dungeon| Some(dungeon.filter_visible_rooms())), //TODO: transition between dungeon floors
                    shop_items: None,
                    ready_timer: None,
                    difficulty: Some(adventure.clone().map_or(1, |adv| adv.difficulty)),
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

fn compute_camera_position(min_x: u32, max_x: u32, min_y: u32, max_y: u32) -> Option<tatami_dungeon::Position> {
    let center_x = (min_x + max_x) / 2;
    let center_y = (min_y + max_y) / 2;

    if min_x <= max_x && min_y <= max_y {
        Some(tatami_dungeon::Position {
            x: center_x,
            y: center_y,
        })
    } else {
        None
    }
}