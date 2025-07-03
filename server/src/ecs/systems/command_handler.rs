// In server/src/ecs/systems/command_handler.rs
use crate::commands::{RpgCommand, PlayerCommand};
use specs::{System, WriteExpect, ReadExpect, Write};
use std::collections::VecDeque;
use crate::ecs::resources::GameState;

// A queue to store commands to be processed
pub type CommandQueue = VecDeque<(String, RpgCommand, bool)>;


pub struct CommandHandlerSystem;

impl<'a> System<'a> for CommandHandlerSystem {
    type SystemData = (
        Write<'a, CommandQueue>,
        ReadExpect<'a, GameState>,
    );

    fn run(&mut self, (mut command_queue, game_state, /* other resources */): Self::SystemData) {
        while let Some((player_name, command, is_priveleged)) = command_queue.pop_front() {
            match *game_state {
                GameState::OutOfDungeon => {
                    match command {
                        RpgCommand::New(class) => {
                            // Create new player character
                            println!("Creating new character for {} with class {:?}", player_name, class);
                            // Implementation here
                        }
                        RpgCommand::Load => {
                            // Load player character
                            println!("Loading character for {}", player_name);
                            // Implementation here
                        }
                        RpgCommand::PlayerCommand(PlayerCommand::Buy(player, item)) => {
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
                        RpgCommand::PlayerCommand(PlayerCommand::Use(player, item)) => {
                            // Handle use command
                            println!("{} is using item {:?}", player_name, item);
                            // Implementation here
                        }
                        _ => {
                            // Command not allowed in this state
                            println!("Command {:?} not allowed in InDungeon state", command);
                        }
                    }
                }
            }
        }
    }
}