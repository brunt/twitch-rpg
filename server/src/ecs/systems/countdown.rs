use crate::ecs::components::{Name, Player};
use crate::ecs::resources::{CountdownTimer, DeltaTime, GameState};
use specs::{Join, Read, ReadStorage, System, WriteExpect};
use std::time;

pub struct CountdownSystem {
    /// The minimum number of players in a lobby before the countdown timer starts.
    pub min_players: usize,
}

impl<'a> System<'a> for CountdownSystem {
    type SystemData = (
        WriteExpect<'a, Option<CountdownTimer>>,
        WriteExpect<'a, GameState>,
        ReadStorage<'a, Player>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, (mut timer, mut game_state, players, delta_time): Self::SystemData) {
        let player_count = players.join().count();
        if matches!(*game_state, GameState::OutOfDungeon)
            && player_count >= self.min_players
            && timer.is_none()
        {
            *timer = Some(CountdownTimer::default());
        }

        if let Some(timer) = timer.as_mut() {
            let elapsed = time::Duration::from_secs_f64(delta_time.0);
            timer.remaining = timer.remaining.saturating_sub(elapsed);
            if timer.remaining.is_zero() {
                *game_state = GameState::InDungeon;
            }
        }
    }
}
