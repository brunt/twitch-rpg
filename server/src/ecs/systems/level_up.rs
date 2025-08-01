use crate::ecs::components::{Experience, Level};
use crate::ecs::resources::{Adventure, GameState};
use pathfinding::num_traits::Pow;
use specs::prelude::*;

pub struct LevelUpSystem;

impl<'a> System<'a> for LevelUpSystem {
    // Fetch both Experience and Level components.
    type SystemData = (
        WriteStorage<'a, Experience>,
        WriteStorage<'a, Level>,
        ReadExpect<'a, Option<Adventure>>,
        ReadExpect<'a, GameState>,
    );

    fn run(&mut self, (mut experiences, mut levels, adventure, game_state): Self::SystemData) {
        if matches!(*game_state, GameState::InTown) {
            return;
        };
        let Some(ref adv) = *adventure else {
            return;
        };

        // Iterate over the entities that have both Experience and Level
        // Here, adjust your reward as needed.
        for (exp, level) in (&mut experiences, &mut levels).join() {
            // Reward the experience based on some difficulty factor.
            // let reward = 1000 * adventure.difficulty;
            // exp.current += reward;

            // Loop to level up as many times as allowed by the accumulated experience.
            while exp.current >= exp.next_level {
                // Remove the threshold amount (or adjust depending on your design)
                exp.current -= exp.next_level;

                // Increase the level
                level.0 += 1;

                // Option: calculate the next level threshold.
                // You can change this formula to suit your progression curve.
                exp.next_level = f32::pow(exp.next_level as f32, 1.45) as u32;

                // Optionally, insert extra logic (e.g., play animation, spawn effects, update other stats, etc.)
                println!("Leveled up! New Level: {}", level.0);
            }
        }
    }
}
