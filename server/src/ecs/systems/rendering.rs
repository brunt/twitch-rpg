use crate::ecs::components::{
    CharacterClass, Health, HealthComponent, Level, Money, Name, Position, Projectile,
};
use crate::ecs::resources::GameState;
use common::{GameSnapShot, PlayerSnapshot};
use specs::{Join, ReadExpect, ReadStorage, System};
use tokio::sync::broadcast::Sender;

pub struct Rendering {
    pub sender: Sender<GameSnapShot>,
}

impl<'a> System<'a> for Rendering {
    type SystemData = (
        ReadStorage<'a, Name>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, HealthComponent>,
        ReadStorage<'a, Projectile>,
        ReadStorage<'a, CharacterClass>,
        ReadStorage<'a, Level>,
        ReadStorage<'a, Money>,
        ReadExpect<'a, GameState>,
        // inventory
    );

    fn run(
        &mut self,
        (names, positions, health, projectiles, character_classes, levels, monies, game_state): Self::SystemData,
    ) {
        let state = &*game_state;
        match state {
            GameState::OutOfDungeon => {
                let mut gs = GameSnapShot {
                    party: Vec::new(),
                    floor_positions: None,
                };
                for (name, health, character_class, level, money) in
                    (&names, &health, &character_classes, &levels, &monies).join()
                {
                    gs.party.push(PlayerSnapshot {
                        name: name.0.clone(), //TODO: not clone?
                        class: character_class.get_player_class(),
                        health: health.0.clone(),
                        level: level.0.clone(),
                        gold: money.0,
                        sprite_name: "wizard1".to_string(), //TODO: you know
                    });
                }
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
