use crate::ecs::components::combat::{
    ActionTimer, AttackComponent, AttackTarget, DefenseComponent, HealthComponent, MeleeAttacker,
};
use crate::ecs::components::form::FormComponent;
use crate::ecs::components::movement::{CanMove, MovementSpeed, TargetPosition};
use crate::ecs::components::{Enemy, Level, Name, Player, Position};
use crate::ecs::resources::{Adventure, RoomCheck};
use common::{Form, Health};
use specs::{Entities, Join, ReadStorage, System, WriteExpect, WriteStorage};
use tatami_dungeon::Position as TatamiPosition;

pub struct RoomExplorationSystem;

impl<'a> System<'a> for RoomExplorationSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Enemy>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Name>,
        WriteStorage<'a, HealthComponent>,
        WriteStorage<'a, MovementSpeed>,
        WriteStorage<'a, CanMove>,
        WriteStorage<'a, TargetPosition>,
        WriteStorage<'a, AttackComponent>,
        WriteStorage<'a, DefenseComponent>,
        WriteStorage<'a, MeleeAttacker>,
        WriteStorage<'a, AttackTarget>,
        WriteStorage<'a, FormComponent>,
        WriteStorage<'a, Level>,
        WriteExpect<'a, Option<Adventure>>,
    );

    fn run(
        &mut self,
        (
            entities,
            players,
            mut enemies,
            mut positions,
            mut names,
            mut healths,
            mut movement_speeds,
            mut can_move,
            mut target_positions,
            mut attack_components,
            mut defense_components,
            mut melee,
            mut attack_targets,
            mut forms,
            mut levels,
            mut adventure,
        ): Self::SystemData,
    ) {
        let Some(adv) = adventure.as_mut() else {
            return;
        };

        let current_room_id = adv.current_room_index;
        let floor = adv.get_current_floor().clone();
        let Some(current_room) = floor.rooms.iter().find(|r| r.id == current_room_id) else {
            return;
        };

        let player_positions: Vec<_> = (&positions, &players, &healths)
            .join()
            .filter(|(_, _, health)| health.0 != Health::Dead)
            .map(|(pos, _, _)| *pos)
            .collect();

        // --- Room Transition and Exploration Logic ---
        // Find the first connection that a player has moved into.
        'outer: for conn in &current_room.connections {
            let Some(next_room) = floor.rooms.iter().find(|r| r.id == conn.id) else {
                continue;
            };

            for player_pos in &player_positions {
                if next_room.contains(&TatamiPosition::from(player_pos)) {
                    // A player has entered a connected room. Process this transition.
                    let next_room_id = next_room.id;

                    // If we've already explored this room, just update the current room and we're done.
                    if adv.explored_rooms.contains(&next_room_id) {
                        adv.current_room_index = next_room_id;
                        return;
                    }

                    // new room discovered
                    adv.current_room_index = next_room_id;
                    adv.explored_rooms.add_child(current_room_id, next_room_id);

                    let enemy_positions = adv.get_room_enemy_data(next_room_id);
                    let player_entities: Vec<_> =
                        (&entities, &players).join().map(|(e, _)| e).collect();

                    for enemy_pos in enemy_positions {
                        let enemy = entities.create();

                        names
                            .insert(enemy, Name::new(format!("E{}", adv.difficulty)))
                            .expect("Failed to insert enemy name");
                        enemies.insert(enemy, Enemy).expect("Failed to be an enemy");
                        levels
                            .insert(enemy, Level(1))
                            .expect("Failed to insert level");
                        healths
                            .insert(enemy, HealthComponent::new_from_difficulty(adv.difficulty))
                            .expect("Failed to add health");
                        positions
                            .insert(enemy, Position::from(&enemy_pos))
                            .expect("Failed to insert position");
                        movement_speeds
                            .insert(enemy, MovementSpeed(1))
                            .expect("Failed to insert movement speed");
                        can_move
                            .insert(enemy, CanMove)
                            .expect("Failed to be able to move");
                        target_positions
                            .insert(enemy, TargetPosition::from(player_pos))
                            .expect("Failed to insert enemy target position");
                        attack_components
                            .insert(
                                enemy,
                                AttackComponent::from_enemy_difficulty(adv.difficulty),
                            )
                            .expect("Failed to insert attack_component");
                        defense_components
                            .insert(
                                enemy,
                                DefenseComponent::from_enemy_difficulty(adv.difficulty),
                            )
                            .expect("Failed to insert defense_component");
                        forms
                            .insert(enemy, FormComponent(Form::Normal))
                            .expect("failed to add form");
                        melee
                            .insert(enemy, MeleeAttacker)
                            .expect("Failed to insert melee");

                        // Target the first available player deterministically.
                        if let Some(target_entity) = player_entities.first() {
                            attack_targets
                                .insert(
                                    enemy,
                                    AttackTarget {
                                        entity: *target_entity,
                                    },
                                )
                                .expect("Failed to assign AttackTarget");
                        }
                    }
                    // Once we've processed a transition, we break the outer loop and stop.
                    break 'outer;
                }
            }
        }
    }
}
