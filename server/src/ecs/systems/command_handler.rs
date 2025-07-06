// In server/src/ecs/systems/command_handler.rs
use crate::commands::PlayerCommand::Use;
use crate::commands::{PlayerCommand, RpgCommand};
use crate::ecs::components::{
    CharacterClass, Equipment, Experience, Faction, HealthComponent, Level, Money, MovementAI,
    MovementAIKind, MovementSpeed, Name, Position, Renderable, Resource, Stats,
};
use crate::ecs::resources::GameState;
use specs::{Entities, LazyUpdate, Read, ReadExpect, System, Write, WriteExpect, WriteStorage};
use std::collections::VecDeque;
use tatami_dungeon::Dungeon;

// A queue to store commands to be processed
pub type CommandQueue = VecDeque<(String, RpgCommand, bool)>;

pub struct CommandHandlerSystem;

impl<'a> System<'a> for CommandHandlerSystem {
    type SystemData = (
        WriteExpect<'a, CommandQueue>,
        WriteExpect<'a, GameState>,
        WriteStorage<'a, Name>,
        WriteStorage<'a, Level>,
        WriteStorage<'a, HealthComponent>,
        WriteStorage<'a, CharacterClass>,
        WriteStorage<'a, Money>,
        Entities<'a>,
        ReadExpect<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (
            mut command_queue,
            mut game_state,
            mut names,
            mut levels,
            mut healths,
            mut classes,
            mut money,
            entities,
            lazy,
        ): Self::SystemData,
    ) {
        while let Some((player_name, command, _is_privileged)) = command_queue.pop_front() {
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

                            let player_entity = entities.create();
                            levels
                                .insert(player_entity, Level::default())
                                .expect("failed to set default level");
                            names
                                .insert(player_entity, Name(player_name))
                                .expect("failed to set name");
                            //TODO: set health based on class
                            healths
                                .insert(player_entity, HealthComponent::default())
                                .expect("failed to set default health");
                            classes
                                .insert(player_entity, CharacterClass::PlayerClass(class))
                                .expect("failed to set class");
                            money
                                .insert(player_entity, Money::default())
                                .expect("failed to set default money");
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
                        }
                        _ => {}
                    }
                    // Handle in-dungeon commands
                    // ...
                }
            }
        }
    }
}
