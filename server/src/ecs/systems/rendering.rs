use tokio::sync::broadcast::Sender;
use specs::{Join, ReadExpect, ReadStorage, System};
use crate::ecs::components::{Health, Position, Projectile};
use crate::ecs::GameSnapShot;
use crate::ecs::resources::GameState;

pub struct Rendering {
    pub sender: Sender<GameSnapShot>
}

impl<'a> System<'a> for Rendering {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Health>,
        ReadStorage<'a, Projectile>,
        ReadExpect<'a, GameState>,
        // inventory
    );
    
    fn run(&mut self, (positions, health, projectiles, game_state): Self::SystemData) {
        let state = &*game_state;
        match state {
            GameState::OutOfDungeon => {
                let gs = GameSnapShot {
                    party: Vec::new(),
                    floor_positions: None
                };
                for (_, _, _) in (&positions, &health, &projectiles).join() {
                    // gs.party.append()
                }
                dbg!("sending out of dungeon");
                _ = self.sender.send(gs);
            }
            GameState::InDungeon => {
                let gs = GameSnapShot {
                    party: Vec::new(),
                    floor_positions: Some(vec![]),
                };
                for (pos, health, proj) in (&positions, &health, &projectiles).join() {
                    // gs.party.append()
                }
                dbg!("sending in dungeon");
                _ = self.sender.send(gs);
            }
        }
    }
}
