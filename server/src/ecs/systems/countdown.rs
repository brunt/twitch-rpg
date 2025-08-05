use crate::ecs::components::class::CharacterClass;
use crate::ecs::components::combat::{
    AttackComponent, AttackTarget, AttackTimer, DefenseComponent, HealthComponent, MeleeAttacker,
    RangedAttacker,
};
use crate::ecs::components::inventory::Equipment;
use crate::ecs::components::movement::{
    CanMove, DesiredTargetPosition, MovementSpeed, TargetPosition,
};
use crate::ecs::components::{DungeonItem, Enemy, Level, Name, Player, Position, Stats};
use crate::ecs::resources::{Adventure, CountdownTimer, DeltaTime, GameState, ShopInventory};
use crate::ecs::systems::pathfinding::PathfindingSystem;
use common::{Form, Health, PlayerClass};
use rand::seq::IndexedRandom;
use specs::{Entities, Join, Read, ReadStorage, System, Write, WriteExpect, WriteStorage};
use std::time;
use crate::ecs::components::form::FormComponent;

pub struct CountdownSystem {
    /// The minimum number of players in a lobby before the countdown timer starts.
    pub min_players: usize,
}

impl<'a> System<'a> for CountdownSystem {
    type SystemData = (
        WriteExpect<'a, Option<CountdownTimer>>,
        WriteExpect<'a, GameState>,
        WriteExpect<'a, Option<Adventure>>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Enemy>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, HealthComponent>,
        ReadStorage<'a, Stats>,
        ReadStorage<'a, Equipment>,
        WriteStorage<'a, Level>,
        WriteStorage<'a, Name>,
        WriteStorage<'a, MovementSpeed>,
        WriteStorage<'a, CanMove>,
        WriteStorage<'a, TargetPosition>,
        WriteStorage<'a, DungeonItem>,
        WriteStorage<'a, AttackComponent>,
        WriteStorage<'a, DefenseComponent>,
        WriteStorage<'a, AttackTarget>,
        WriteStorage<'a, MeleeAttacker>,
        WriteStorage<'a, RangedAttacker>,
        WriteStorage<'a, FormComponent>,
        Write<'a, ShopInventory>,
        Read<'a, DeltaTime>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (
            mut game_timer,
            mut game_state,
            mut adventure,
            players,
            mut enemies,
            mut positions,
            mut healths,
            stats,
            equipment,
            mut levels,
            mut names,
            mut movementspeeds,
            mut can_move,
            mut targets,
            mut dungeon_items,
            mut attack_components,
            mut defense_components,
            mut attack_targets,
            mut melee,
            mut ranged,
            mut forms,
            mut shop_inventory,
            delta_time,
            mut entities,
        ): Self::SystemData,
    ) {
        let player_count = players.join().count();

        if matches!(*game_state, GameState::InTown)
            && player_count >= self.min_players
            && game_timer.is_none()
        {
            *game_timer = Some(CountdownTimer::default());
        }

        if let Some(timer) = game_timer.as_mut() {
            let elapsed = time::Duration::from_secs_f64(delta_time.0);
            timer.remaining = timer.remaining.saturating_sub(elapsed);

            // start the dungeon! a lot of logic is here, perhaps it should move somewhere
            if timer.remaining.is_zero() {
                let adv = Adventure::default();
                // insert everything from dungeon into ECS

                for (entity, _player) in (&entities, &players).join() {
                    let equipped_items = equipment.get(entity).expect("failed to read equipment");

                    // add positions to ECS
                    positions
                        .insert(entity, Position::from(&adv.dungeon.player_position))
                        .expect("Failed to insert player position");
                    movementspeeds
                        .insert(entity, MovementSpeed::from_items(equipped_items))
                        .expect("Failed to insert movement speed");
                    can_move
                        .insert(entity, CanMove)
                        .expect("Failed to insert can_move");

                    let st = stats.get(entity).expect("failed to read stats");
                    let equipped_items = equipment.get(entity).expect("failed to read equipment");
                    attack_components
                        .insert(
                            entity,
                            AttackComponent::from_stats_and_items(st, equipped_items),
                        )
                        .expect("failed to add attack_component");
                    defense_components
                        .insert(
                            entity,
                            DefenseComponent {
                                defense: 0,
                                evasion: 0,
                            },
                        )
                        .expect("failed to add defense_component");
                    forms.insert(entity, FormComponent(Form::Normal)).expect("failed to add form");
                    //TODO: conditionally set melee/range based on equipped item
                    // melee
                    //     .insert(entity, MeleeAttacker)
                    //     .expect("failed to add melee");
                    ranged
                        .insert(entity, RangedAttacker)
                        .expect("failed to add ranged");
                }

                let player_entities: Vec<_> =
                    (&entities, &players).join().map(|(e, _)| e).collect();

                adv.get_room_enemy_data(adv.current_room_index)
                    .iter()
                    .for_each(|pos| {
                        let enemy = entities.create();
                        names
                            .insert(enemy, Name::new(format!("E{}", adv.difficulty))) //TODO: use dungeon floor enemy difficulty variance
                            .expect("Failed to insert enemy name");
                        enemies.insert(enemy, Enemy).expect("Failed to be an enemy");
                        levels
                            .insert(enemy, Level(1))
                            .expect("Failed to insert level");
                        healths
                            .insert(enemy, HealthComponent::new_from_difficulty(adv.difficulty))
                            .expect("Failed to add health");
                        positions
                            .insert(enemy, Position::from(pos))
                            .expect("Failed to insert enemy position");
                        movementspeeds
                            .insert(enemy, MovementSpeed(1))
                            .expect("Failed to insert movement speed");
                        can_move
                            .insert(enemy, CanMove)
                            .expect("Failed to insert can_move");
                        targets
                            .insert(enemy, TargetPosition::from(&adv.dungeon.player_position))
                            .expect("Failed to insert enemy target position");
                        attack_components
                            .insert(
                                enemy,
                                AttackComponent::from_enemy_difficulty(adv.difficulty),
                            )
                            .expect("failed to add attack_component");
                        forms.insert(enemy, FormComponent(Form::Normal)).expect("failed to add form");
                        defense_components
                            .insert(
                                enemy,
                                DefenseComponent {
                                    defense: 0,
                                    evasion: 0,
                                },
                            )
                            .expect("failed to add defense_component");
                        melee
                            .insert(enemy, MeleeAttacker)
                            .expect("failed to add melee");

                        if !player_entities.is_empty() {
                            let target_entity = *player_entities
                                .choose(&mut rand::rngs::ThreadRng::default())
                                .expect("RNG Failed to choose player");
                            attack_targets
                                .insert(
                                    enemy,
                                    AttackTarget {
                                        entity: target_entity,
                                    },
                                )
                                .expect("Failed to assign AttackTarget");
                        }
                    });

                adv.get_visible_item_data().iter().for_each(|pos| {
                    let item = entities.create();
                    positions
                        .insert(item, Position::from(pos))
                        .expect("failed to insert item");
                    dungeon_items
                        .insert(item, DungeonItem)
                        .expect("failed to be an item");
                });

                *adventure = Some(adv);
                *game_state = GameState::OnAdventure;
                *game_timer = None;
                *shop_inventory = ShopInventory::default();
            }
        }
    }
}
