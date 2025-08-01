use crate::ecs::components::combat::{
    AttackComponent, AttackTarget, AttackTimer, FiredProjectile, HealthComponent, MeleeAttacker,
    RangedAttacker,
};
use crate::ecs::components::inventory::Equipment;
use crate::ecs::components::movement::{CanMove, MovementSpeed, Path, TargetPosition};
use crate::ecs::components::{Experience, Level, Money, Name, Player, Position};
use crate::ecs::resources::{Adventure, GameState, RoomCheck};
use specs::{Entities, Join, ReadStorage, System, WriteExpect, WriteStorage};
use std::ops::Deref;

pub struct DungeonComplete;

impl<'a> System<'a> for DungeonComplete {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameState>,
        WriteExpect<'a, Option<Adventure>>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Name>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Experience>,
        WriteStorage<'a, Money>,
        WriteStorage<'a, Equipment>,
        WriteStorage<'a, TargetPosition>,
        WriteStorage<'a, MovementSpeed>,
        WriteStorage<'a, HealthComponent>,
        WriteStorage<'a, AttackTimer>,
        WriteStorage<'a, AttackComponent>,
        WriteStorage<'a, CanMove>,
        WriteStorage<'a, RangedAttacker>,
        WriteStorage<'a, MeleeAttacker>,
        WriteStorage<'a, Path>,
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
            mut paths,
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
        // if a player is at a stair position, change the gamestate
        let mut pos_to_remove = vec![];
        let mut players_to_remove = vec![];
        for (entity, _, position) in (&entities, &players, &positions).join() {
            for stair_position in &stair_positions {
                if position == stair_position {
                    *game_state = GameState::AfterDungeon;

                    // remove relevant components
                    pos_to_remove.push(entity);
                    target_positions.remove(entity);
                    movementspeeds.remove(entity);
                    can_move.remove(entity);
                    paths.remove(entity);
                    fired_projectiles.remove(entity);
                    ranged_attackers.remove(entity);
                    melee_attackers.remove(entity);
                    healths.remove(entity);
                    attack_components.remove(entity);
                    attack_timer.remove(entity);
                    // dole out rewards

                    // TODO: does this play with the level up system?
                    if let Some(mut exp) = experiences.get_mut(entity) {
                        exp.current += 100 * adv.difficulty;
                    }

                    // TODO: other multipliers
                    if let Some(mut money) = monies.get_mut(entity) { 
                        money.0 += 100 * adv.difficulty;
                    }

                    // TODO: save character data

                    // remove players from party
                    players_to_remove.push(entity);
                    names.remove(entity);

                    // change game state to InTown
                    *game_state = GameState::InTown;
                }
            }
        }

        for pos in pos_to_remove {
            positions.remove(pos);
        }
        for p in players_to_remove {
            players.remove(p);
        }
    }
}
