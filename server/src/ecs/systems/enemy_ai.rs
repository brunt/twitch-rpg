use crate::ecs::components::{Enemy, Player, Position};
use crate::ecs::components::movement::TargetPosition;
use specs::{Entities, Join, Read, ReadStorage, System, WriteStorage};

pub struct EnemyAISystem;

impl<'a> System<'a> for EnemyAISystem {
    type SystemData = (
        ReadStorage<'a, Enemy>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, TargetPosition>,
        Entities<'a>
    );

    fn run(&mut self, (enemies, players, positions, mut targets, entities): Self::SystemData) {
        let player_positions: Vec<&Position> =
            (&positions, &players, &entities).join().map(|(p, _, _)| p).collect();

        for (enemy_pos, _, enemy_entity) in (&positions, &enemies, &entities).join() {
            // Find closest player
            if let Some(closest_player_pos) = player_positions
                .iter()
                .min_by_key(|player_pos| enemy_pos.distance_to(player_pos))
            {
                // Set the target to the player's current position
                targets.insert(enemy_entity, TargetPosition::from(*closest_player_pos)).expect("Failed to insert enemy target position");
            }
        }
    }
}
