use crate::ecs::components::{Name, Player};
use crate::ecs::resources::{Adventure, CountdownTimer, DeltaTime, GameState};
use specs::{Join, Read, ReadStorage, System, WriteExpect};
use std::time;
use tatami_dungeon::Tile;

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
        Read<'a, DeltaTime>,
    );

    fn run(
        &mut self,
        (mut game_timer, mut game_state, mut dungeon, players, delta_time): Self::SystemData,
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
                // generate a dungeon and add it to ECS here
                *dungeon = Some(Adventure::default());
                *game_state = GameState::OnAdventure;
                *game_timer = None;
            }
        }
    }
}
