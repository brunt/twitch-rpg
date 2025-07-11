use std::time;
use crate::ecs::components::{Name, Player};
use crate::ecs::resources::{CountdownTimer, DeltaTime, GameState};
use specs::{Join, Read, ReadStorage, System, WriteExpect};

pub struct CountdownSystem;

impl<'a> System<'a> for CountdownSystem {
    type SystemData = (
        WriteExpect<'a, CountdownTimer>,
        WriteExpect<'a, GameState>,
        ReadStorage<'a, Player>,
        Read<'a, DeltaTime>,
    );

    fn run(&mut self, (mut timer, mut game_state, players, delta_time): Self::SystemData) {
        let player_count = players.join().count();
        if matches!(*game_state, GameState::OutOfDungeon) && player_count > 0 && !timer.active {
            timer.active = true;
            timer.remaining = time::Duration::from_secs(120);
        }
        
        if timer.active {
            let delta_secs = delta_time.0;
            let elapsed = time::Duration::new(delta_secs, 0);

            if timer.remaining > elapsed {
                timer.remaining -= elapsed;
            } else {
                timer.remaining = time::Duration::from_secs(0);
                timer.active = false;
                *game_state = GameState::InDungeon; //TODO: should this happen here?
            }


        }
        
    }
}
