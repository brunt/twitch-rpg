use crate::ecs::components::combat::{AttackTarget, HealthComponent};
use crate::ecs::components::movement::{DesiredTargetPosition, TargetPosition};
use crate::ecs::components::{DungeonItem, Enemy, Opened, Player, Position};
use crate::ecs::resources::{Adventure, GroupDestination, RoomCheck};
use common::Health;
use specs::prelude::*;

pub struct PlayerAI;

impl<'a> System<'a> for PlayerAI {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, DesiredTargetPosition>,
        WriteStorage<'a, AttackTarget>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Enemy>,
        ReadStorage<'a, HealthComponent>,
        ReadStorage<'a, DungeonItem>,
        ReadStorage<'a, Opened>,
        ReadExpect<'a, Option<Adventure>>,
        WriteExpect<'a, GroupDestination>,
    );

    /// Player Pathfinding:
    /// * When in a room, target enemies 1 at a time until they're gone
    /// * Then target any treasure in the room
    /// * Then target stairs if any
    /// * Then target a corridor to go to another room
    fn run(
        &mut self,
        (
            entities,
            positions,
            mut targets,
            mut attack_targets,
            players,
            enemies,
            healths,
            dungeon_items,
            opened,
            adv,
            mut group_destination,
        ): Self::SystemData,
    ) {
        let Some(adv) = adv.as_ref() else {
            return;
        };

        for (player_entity, pos, _) in (&entities, &positions, &players).join() {
            let Some(current_room) = adv
                .get_current_floor()
                .rooms
                .iter()
                .find(|r| r.id == adv.current_room_index)
            else {
                return;
            };

            // Priority 1: Enemies
            let enemies_in_room: Vec<(Entity, &Position)> =
                (&entities, &enemies, &healths, &positions)
                    .join()
                    .filter(|(enemy, _, health, pos)| {
                        !matches!(health.0, Health::Dead)
                            && current_room.contains(&tatami_dungeon::Position::from(*pos))
                    })
                    .map(|(enemy, _, _, pos)| (enemy, pos))
                    .collect();

            let items_in_room: Vec<&Position> =
                (&entities, &dungeon_items, &positions, opened.maybe())
                    .join()
                    .filter(|(_, _, pos, opened_maybe)| {
                        opened_maybe.is_none()
                            && current_room.contains(&tatami_dungeon::Position::from(*pos))
                    })
                    .map(|(_, _, pos, _)| pos)
                    .collect();

            if let Some((enemy_id, enemy_pos)) = enemies_in_room.first() {
                let enemy_dungeon_pos = tatami_dungeon::Position::from(*enemy_pos);
                let player_dungeon_pos = tatami_dungeon::Position::from(pos);

                let floor = adv.get_current_floor();
                let map_dimensions = (floor.tiles[0].len() as u32, floor.tiles.len() as u32);

                let adjacents: Vec<_> = enemy_dungeon_pos
                    .adjacent_8(map_dimensions)
                    .into_iter()
                    .filter(|adj| {
                        let x = adj.x as usize;
                        let y = adj.y as usize;
                        floor
                            .tiles
                            .get(y)
                            .and_then(|row| row.get(x))
                            .is_some_and(|tile| matches!(tile, tatami_dungeon::Tile::Floor))
                    })
                    .collect();

                attack_targets
                    .insert(player_entity, AttackTarget { entity: *enemy_id })
                    .expect("failed to add attack target");
                if let Some(target_adj) = adjacents
                    .into_iter()
                    .min_by_key(|adj| adj.distance(player_dungeon_pos))
                {
                    targets
                        .insert(
                            player_entity,
                            DesiredTargetPosition {
                                x: target_adj.x,
                                y: target_adj.y,
                            },
                        )
                        .expect("Failed to insert target position");
                } else {
                    targets
                        .insert(
                            player_entity,
                            DesiredTargetPosition {
                                x: enemy_pos.x,
                                y: enemy_pos.y,
                            },
                        )
                        .expect("Failed to insert target position");
                }
            }
            // Priority 2: Items
            else if let Some(item_pos) = items_in_room.first() {
                targets
                    .insert(
                        player_entity,
                        DesiredTargetPosition {
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
                        DesiredTargetPosition {
                            x: stair.position.x,
                            y: stair.position.y,
                        },
                    )
                    .expect("Failed to insert target position");
            }
            // Priority 4: Connection doors
            else if let Some(group_dest) = group_destination.target_room_id {
                // if there's a group destination, go there.
                if let Some(next_room) = adv
                    .get_current_floor()
                    .rooms
                    .iter()
                    .find(|r| r.id == group_dest)
                {
                    targets
                        .insert(
                            player_entity,
                            DesiredTargetPosition::from(&next_room.center()),
                        )
                        .expect("Failed to insert target position");
                }
            } else {
                // Priority 5: Find a new destination (unexplored room or backtrack)
                let unexplored_connection = current_room
                    .connections
                    .iter()
                    .find(|conn| !adv.explored_rooms.contains(&conn.id));

                if let Some(conn) = unexplored_connection {
                    // Found an unexplored path, set it as the destination.
                    group_destination.target_room_id = Some(conn.id);
                } else {
                    // No unexplored paths from here, we need to backtrack.
                    let floor = adv.get_current_floor();
                    if let Some(backtrack_room_id) = adv
                        .explored_rooms
                        .find_next_room_for_dfs(current_room.id, floor)
                    {
                        group_destination.target_room_id = Some(backtrack_room_id);
                    }
                }
            }
        }
    }
}
