use crate::ecs::components::Player;
use crate::ecs::components::combat::HealthComponent;
use crate::ecs::resources::{Adventure, GameState};
use common::Health;
use specs::{Entities, Join, ReadStorage, System, WriteExpect};

pub struct PartyWipeSystem;

impl<'a> System<'a> for PartyWipeSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, HealthComponent>,
        WriteExpect<'a, GameState>,
        WriteExpect<'a, Option<Adventure>>,
    );

    fn run(
        &mut self,
        (entities, players, healths, mut game_state, mut adventure): Self::SystemData,
    ) {
        if !matches!(*game_state, GameState::OnAdventure) {
            return;
        }

        let mut player_entities_to_delete = Vec::new();

        let all_players_are_dead = (&entities, &players, &healths)
            .join()
            .inspect(|(entity, _, _)| player_entities_to_delete.push(*entity))
            .all(|(_, _, health)| matches!(health.0, Health::Dead));

        if !player_entities_to_delete.is_empty() && all_players_are_dead {
            //TODO: delete players from database
            *game_state = GameState::InTown;
            *adventure = None;
            for player_entity in player_entities_to_delete {
                entities
                    .delete(player_entity)
                    .expect("Failed to delete player entity after party wipe");
            }
        }
    }
}
