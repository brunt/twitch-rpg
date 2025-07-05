// In server/src/ecs/systems/command_handler.rs
use crate::commands::{PlayerCommand, RpgCommand};
use crate::ecs::components::{
    CharacterClass, Equipment, Experience, Faction, Health, MovementAI, MovementAIKind,
    MovementSpeed, PlayerClass, Position, Renderable, Resource, Stats,
};
use crate::ecs::entities::create_player_entity;
use crate::ecs::resources::GameState;
use specs::{Entities, LazyUpdate, Read, ReadExpect, System, Write, WriteExpect, WriteStorage};
use std::collections::VecDeque;
use tatami_dungeon::Dungeon;
use crate::commands::PlayerCommand::Use;

// A queue to store commands to be processed
pub type CommandQueue = VecDeque<(String, RpgCommand, bool)>;

pub struct CommandHandlerSystem;

impl<'a> System<'a> for CommandHandlerSystem {
    type SystemData = (
        WriteExpect<'a, CommandQueue>,
        WriteExpect<'a, GameState>,
        Entities<'a>,
        ReadExpect<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (
            mut command_queue,
            mut game_state,
            entities,
            lazy,
        ): Self::SystemData,
    ) {
        while let Some((player_name, command, is_privileged)) = command_queue.pop_front() {
            match *game_state {
                GameState::OutOfDungeon => {
                    match command {
                        RpgCommand::Join(class) => {
                            // a player has queued up to join a dungeon.
                            // they begin in "town" and are able to immediately purchase items
                            // when the dungeon starts, they are added as entities (with purchased items) to dungeon
                            // before that, they are stored in the Town hashmap
                            println!(
                                "Creating new character for {} with class {:?}",
                                player_name, class
                            );

                            // Create the player entity with appropriate components
                            // let player_entity = entities.create();
                            let player_entity = create_player_entity(
                                &entities,
                                &lazy,
                                player_name.clone(),
                                class.clone(),
                            );
                        }
                        // Other commands...
                        RpgCommand::Rejoin => {
                            // Load player character
                            println!("Loading character for {}", player_name);
                            // Implementation here
                        }
                        RpgCommand::PlayerCommand(PlayerCommand::Buy(item)) => {
                            // Handle buy command
                            println!("{} is buying item {:?}", player_name, item);
                            // Implementation here
                        }
                        _ => {
                            // Command not allowed in this state
                            println!("Command {:?} not allowed in OutOfDungeon state", command);
                        }
                    }
                }
                GameState::InDungeon => {
                    match command {
                        RpgCommand::PlayerCommand(Use(item)) => {
                            println!("{} is using item {:?}", player_name, item);
                        },
                        _ => {}
                    }
                    // Handle in-dungeon commands
                    // ...
                }
            }
        }
    }
}
