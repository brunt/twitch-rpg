use crate::ecs::components::movement::TargetPosition;
use crate::ecs::components::{DungeonItem, Enemy, Player, Position};
use crate::ecs::resources::{Adventure, RoomCheck};
use specs::prelude::*;
use tatami_dungeon::{Dungeon, Enemy as TatamiEnemy};

pub struct PlayerAI;

impl<'a> System<'a> for PlayerAI {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, TargetPosition>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Enemy>,
        ReadStorage<'a, DungeonItem>,
        ReadExpect<'a, Option<Adventure>>,
    );

    /// Player Pathfinding:
    /// * When in a room, target enemies 1 at a time until they're gone
    /// * Then target any treasure in the room
    /// * Then target stairs if any
    /// * Then target a corridor to go to another room
    fn run(
        &mut self,
        (entities, positions, mut targets, players, enemies, dungeon_items, adv): Self::SystemData,
    ) {
        let Some(adv) = adv.as_ref() else {
            return;
        };

        for (player_entity, pos, _) in (&entities, &positions, &players).join() {
            let Some(current_room) = adv.dungeon.floors[adv.current_floor_index]
                .rooms
                .iter()
                .find(|r| r.id == adv.current_room_index)
            else {
                return;
            };
            
            // Priority 1: Enemies
            let enemies_in_room: Vec<&Position> =
                (&entities, &enemies, &positions)
                    .join()
                    .filter(|(_, _, pos)|
                        current_room.contains(&tatami_dungeon::Position::from(*pos))
                    )
                    .map(|(_, _, pos)| pos)
                    .collect();

            let items_in_room: Vec<&Position> = (&entities, &dungeon_items, &positions).join()
                .filter(|(_, _, pos)|
                    current_room.contains(&tatami_dungeon::Position::from(*pos))
                )
                .map(|(_, _, pos)| pos)
                .collect();
            
            if let Some(enemy_pos) = enemies_in_room.first() {
                targets
                    .insert(
                        player_entity,
                        TargetPosition {
                            x: enemy_pos.x,
                            y: enemy_pos.y,
                        },
                    )
                    .expect("Failed to insert target position");
            }
            // Priority 2: Items
            else if let Some(item_pos) = items_in_room.first() {
                targets
                    .insert(
                        player_entity,
                        TargetPosition {
                            x: item_pos.x,
                            y: item_pos.y,
                        },
                    )
                    .expect("Failed to insert target position");
            }
            // Priority 3: Downwards stairs
            else if let Some(stair) = current_room.stairs.iter().find(|s| s.downwards) {
                targets
                    .insert(
                        player_entity,
                        TargetPosition {
                            x: stair.position.x,
                            y: stair.position.y,
                        },
                    )
                    .expect("Failed to insert target position");
            }
            // Priority 4: Connection doors
            else if let Some(conn) = current_room.connections.first() {
                targets
                    .insert(
                        player_entity,
                        TargetPosition {
                            x: conn.door.x,
                            y: conn.door.y,
                        },
                    )
                    .expect("Failed to insert target position");
            }
        }
    }
}
