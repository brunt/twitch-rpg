use crate::ecs::components::movement::{MovementSpeed, TargetPosition};
use crate::ecs::components::{Enemy, HealthComponent, Level, Name, Player, Position, Stats};
use crate::ecs::resources::{Adventure, CountdownTimer, DeltaTime, GameState, ShopInventory};
use crate::ecs::systems::pathfinding::PathfindingSystem;
use common::Health;
use specs::{Entities, Join, Read, ReadStorage, System, Write, WriteExpect, WriteStorage};
use std::time;

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
        WriteStorage<'a, Level>,
        WriteStorage<'a, Name>,
        WriteStorage<'a, MovementSpeed>,
        WriteStorage<'a, TargetPosition>,
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
            mut levels,
            mut names,
            mut movementspeeds,
            mut targets,
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
            if timer.remaining.is_zero() {
                let adv = Adventure::default();
                // insert everything from dungeon into ECS

                for (entity, _player) in (&entities, &players).join() {
                    // add positions to ECS
                    positions
                        .insert(
                            entity,
                            Position {
                                x: adv.dungeon.player_position.x,
                                y: adv.dungeon.player_position.y,
                            },
                        )
                        .expect("Failed to insert player position");
                    movementspeeds
                        .insert(entity, MovementSpeed(1))
                        .expect("Failed to insert movement speed");
                }

                adv.get_enemy_data().iter().for_each(|pos| {
                    let enemy = entities.create();
                    names
                        .insert(enemy, Name::default())
                        .expect("Failed to insert enemy name");
                    enemies.insert(enemy, Enemy).expect("Failed to be an enemy");
                    levels
                        .insert(enemy, Level(1))
                        .expect("Failed to insert level");
                    healths
                        .insert(enemy, HealthComponent(Health::Alive { hp: 10, max_hp: 10 }))
                        .expect("Failed to add health");
                    positions
                        .insert(enemy, Position { x: pos.x, y: pos.y })
                        .expect("Failed to insert enemy position");
                    movementspeeds
                        .insert(enemy, MovementSpeed(1))
                        .expect("Failed to insert movement speed");
                });

                *adventure = Some(adv);
                *game_state = GameState::OnAdventure;
                *game_timer = None;
                *shop_inventory = ShopInventory::default();
            }
        }
    }
}
