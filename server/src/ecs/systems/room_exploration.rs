use crate::ecs::components::combat::{
    AttackComponent, AttackTarget, DefenseComponent, HealthComponent, MeleeAttacker,
};
use crate::ecs::components::movement::{MovementSpeed, PrevPosition, TargetPosition};
use crate::ecs::components::{Enemy, Level, Name, Player, Position};
use crate::ecs::resources::{Adventure, DirectionOffset, RoomCheck};
use common::Health;
use rand::seq::IndexedRandom;
use specs::{Entities, Join, ReadStorage, System, WriteExpect, WriteStorage};
use tatami_dungeon::Position as TatamiPosition;

pub struct RoomExplorationSystem;

impl<'a> System<'a> for RoomExplorationSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Enemy>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, PrevPosition>,
        WriteStorage<'a, Name>,
        WriteStorage<'a, HealthComponent>,
        WriteStorage<'a, MovementSpeed>,
        WriteStorage<'a, TargetPosition>,
        WriteStorage<'a, AttackComponent>,
        WriteStorage<'a, DefenseComponent>,
        WriteStorage<'a, MeleeAttacker>,
        WriteStorage<'a, AttackTarget>,
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
            mut prev_positions,
            mut names,
            mut healths,
            mut movement_speeds,
            mut target_positions,
            mut attack_components,
            mut defense_components,
            mut melee,
            mut attack_targets,
            mut levels,
            mut adventure,
        ): Self::SystemData,
    ) {
        let Some(adv) = adventure.as_mut() else { return; };

        let current_room_id = adv.current_room_index;
        let floor = adv.get_current_floor().clone();
        let Some(current_room) = floor.rooms.iter().find(|r| r.id == current_room_id) else { return };

        let player_positions: Vec<_> = (&positions, &players)
            .join()
            .map(|(pos, _)| pos.clone())
            .collect();
        
        for player_pos in player_positions {
            for conn in &current_room.connections {
                let Some(next_room) = floor.rooms.iter().find(|r| r.id == conn.id) else { continue };

                if next_room.contains(&TatamiPosition::from(&player_pos)) {
                    if adv.explored_rooms.contains(&next_room.id) {
                        adv.current_room_index = next_room.id;
                        return;
                    }

                    let enemy_positions = adv.get_room_enemy_data(next_room.id);
                    let player_entities: Vec<_> = (&entities, &players).join().map(|(e, _)| e).collect();

                    adv.current_room_index = next_room.id;

                    for enemy_pos in enemy_positions {
                        let enemy = entities.create();

                        names
                            .insert(enemy, Name::new(format!("E{}", adv.difficulty)))
                            .expect("Failed to insert enemy name");
                        enemies.insert(enemy, Enemy).expect("Failed to be an enemy");
                        levels.insert(enemy, Level(1)).expect("Failed to insert level");
                        healths
                            .insert(enemy, HealthComponent(Health::Alive { hp: 2, max_hp: 2 }))
                            .expect("Failed to add health");
                        positions
                            .insert(enemy, Position::from(&enemy_pos))
                            .expect("Failed to insert position");
                        movement_speeds
                            .insert(enemy, MovementSpeed(1))
                            .expect("Failed to insert movement speed");
                        target_positions.insert(enemy, TargetPosition::from(&player_pos)).expect("Failed to insert enemy target position");
                        attack_components
                            .insert(
                                enemy,
                                AttackComponent {
                                    damage: 1,
                                    hit_rating: 1,
                                    range: 1,
                                    cooldown: 3000,
                                },
                            )
                            .expect("Failed to insert attack_component");
                        defense_components
                            .insert(
                                enemy,
                                DefenseComponent {
                                    defense: 0,
                                    evasion: 0,
                                },
                            )
                            .expect("Failed to insert defense_component");
                        melee.insert(enemy, MeleeAttacker).expect("Failed to insert melee");

                        if let Some(&target_entity) = player_entities.choose(&mut rand::rngs::ThreadRng::default()) {
                            attack_targets
                                .insert(enemy, AttackTarget { entity: target_entity })
                                .expect("Failed to assign AttackTarget");
                        }
                    }

                    adv.explored_rooms.insert(conn.id);
                    return;
                }
            }
        }
    }
}