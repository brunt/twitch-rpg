use crate::ecs::components::combat::{
    ActionTimer, AttackComponent, AttackTarget, FiredProjectile, HealthComponent, MeleeAttacker,
    RangedAttacker,
};
use crate::ecs::components::inventory::Equipment;
use crate::ecs::components::movement::{CanMove, MovementSpeed, TargetPosition};
use crate::ecs::components::{Experience, Level, Money, Name, Player, Position};
use crate::ecs::resources::{Adventure, GameState};
use crate::ecs::shop::{ShopItemPool, initialize_reward_items, initialize_shop_items};
use specs::{Entities, Join, System, WriteExpect, WriteStorage};

pub struct DungeonComplete;

impl<'a> System<'a> for DungeonComplete {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameState>,
        WriteExpect<'a, Option<Adventure>>,
        WriteExpect<'a, ShopItemPool>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Name>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Experience>,
        WriteStorage<'a, Money>,
        WriteStorage<'a, Equipment>,
        WriteStorage<'a, TargetPosition>,
        WriteStorage<'a, MovementSpeed>,
        WriteStorage<'a, HealthComponent>,
        WriteStorage<'a, ActionTimer>,
        WriteStorage<'a, AttackComponent>,
        WriteStorage<'a, CanMove>,
        WriteStorage<'a, RangedAttacker>,
        WriteStorage<'a, MeleeAttacker>,
        WriteStorage<'a, FiredProjectile>,
    );
    // TODO: readstorage from stats to display? or is that in rendering system?
    // TODO: for now, check if a player is on a stair tile and call it a day

    // TODO: add a 20ish second timer before going back to InTown
    // eventually, dungeons will have multiple floors and we check here for only the last.
    fn run(
        &mut self,
        (
            mut entities,
            mut game_state,
            mut adventure,
            mut shop_item_pool,
            mut players,
            mut names,
            mut positions,
            mut experiences,
            mut monies,
            mut equipments,
            mut target_positions,
            mut movementspeeds,
            mut healths,
            mut attack_timer,
            mut attack_components,
            mut can_move,
            mut ranged_attackers,
            mut melee_attackers,
            mut fired_projectiles,
        ): Self::SystemData,
    ) {
        if !matches!(*game_state, GameState::OnAdventure) {
            return;
        };
        let Some(adv) = adventure.as_mut() else {
            return;
        };

        let stair_positions: Vec<Position> = adv
            .get_current_floor()
            .rooms
            .iter()
            .flat_map(|room| {
                room.stairs.iter().filter_map(|stair| {
                    if stair.downwards {
                        Some(Position::from(&stair.position.clone()))
                    } else {
                        None
                    }
                })
            })
            .collect();
        // Check if any player is at a stair position
        let any_player_at_stairs =
            (&entities, &players, &positions)
                .join()
                .any(|(_, _, position)| {
                    stair_positions
                        .iter()
                        .any(|stair_position| position == stair_position)
                });

        // If any player reached stairs, process all players
        if any_player_at_stairs {
            *game_state = GameState::AfterDungeon;

            let mut pos_to_remove = vec![];
            let mut players_to_remove = vec![];

            for (entity, _, _) in (&entities, &players, &positions).join() {
                // remove relevant components
                pos_to_remove.push(entity);
                target_positions.remove(entity);
                movementspeeds.remove(entity);
                can_move.remove(entity);
                // paths.remove(entity);
                fired_projectiles.remove(entity);
                ranged_attackers.remove(entity);
                melee_attackers.remove(entity);
                healths.remove(entity);
                attack_components.remove(entity);
                attack_timer.remove(entity);

                // dole out rewards
                let items = initialize_reward_items(adv.difficulty);

                // TODO: does this play with the level up system?
                if let Some(mut exp) = experiences.get_mut(entity) {
                    exp.current += 100 * adv.difficulty;
                }

                // TODO: other multipliers
                if let Some(mut money) = monies.get_mut(entity) {
                    money.0 += (100 * adv.difficulty).max(100);
                }

                // TODO: save character data after loot is given (end of a timer?)

                // remove players from party
                players_to_remove.push(entity);
                names.remove(entity);
            }

            for pos in pos_to_remove {
                positions.remove(pos);
            }
            for p in players_to_remove {
                players.remove(p);
            }

            // regenerate shop items
            shop_item_pool.all_items = initialize_shop_items();

            // change game state to InTown
            *game_state = GameState::InTown;
        }
    }
}
