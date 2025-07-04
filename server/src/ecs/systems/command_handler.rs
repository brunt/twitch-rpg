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

// A queue to store commands to be processed
pub type CommandQueue = VecDeque<(String, RpgCommand, bool)>;

pub struct CommandHandlerSystem;

impl<'a> System<'a> for CommandHandlerSystem {
    type SystemData = (
        WriteExpect<'a, CommandQueue>,
        WriteExpect<'a, GameState>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, MovementSpeed>,
        WriteStorage<'a, CharacterClass>,
        WriteStorage<'a, Health>,
        WriteStorage<'a, Equipment>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, Resource>,
        WriteStorage<'a, Stats>,
        WriteStorage<'a, Experience>,
        WriteStorage<'a, MovementAI>,
        WriteStorage<'a, Faction>,
        ReadExpect<'a, Dungeon>,
        ReadExpect<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (
            mut command_queue,
            mut game_state,
            entities,
            mut positions,
            mut speeds,
            mut character_classes,
            mut healths,
            mut equipments,
            mut renderables,
            mut resources,
            mut stats,
            mut experiences,
            mut movement_ais,
            mut factions,
            dungeon,
            lazy,
        ): Self::SystemData,
    ) {
        while let Some((player_name, command, is_privileged)) = command_queue.pop_front() {
            match *game_state {
                GameState::OutOfDungeon => {
                    match command {
                        RpgCommand::New(class) => {
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

                            // Base components for all players
                            // positions.insert(player_entity, starting_pos).unwrap();
                            // speeds.insert(player_entity, MovementSpeed(3)).unwrap(); // Base movement speed
                            // character_classes
                            //     .insert(
                            //         player_entity,
                            //         CharacterClass {
                            //             class: class.clone(),
                            //             level: 1,
                            //         },
                            //     )
                            //     .unwrap();
                            // 
                            // // Health varies by class
                            // let base_hp = match class {
                            //     PlayerClass::Fighter | PlayerClass::Paladin => 15,
                            //     PlayerClass::Monk | PlayerClass::Ranger => 12,
                            //     PlayerClass::Rogue | PlayerClass::Druid | PlayerClass::Cleric => 10,
                            //     PlayerClass::Wizard | PlayerClass::Warlock => 8,
                            // };
                            // healths
                            //     .insert(
                            //         player_entity,
                            //         Health::Alive {
                            //             hp: base_hp,
                            //             max_hp: base_hp,
                            //         },
                            //     )
                            //     .unwrap();
                            // 
                            // // Empty equipment
                            // equipments
                            //     .insert(
                            //         player_entity,
                            //         Equipment {
                            //             weapon: None,
                            //             armor: None,
                            //         },
                            //     )
                            //     .unwrap();
                            // 
                            // //TODO: refactor this out
                            // // Class-specific starting stats
                            // let player_stats = match class {
                            //     PlayerClass::Fighter => Stats {
                            //         strength: 14,
                            //         agility: 10,
                            //         intelligence: 8,
                            //         vitality: 12,
                            //     },
                            //     PlayerClass::Rogue => Stats {
                            //         strength: 10,
                            //         agility: 14,
                            //         intelligence: 10,
                            //         vitality: 8,
                            //     },
                            //     PlayerClass::Wizard => Stats {
                            //         strength: 6,
                            //         agility: 8,
                            //         intelligence: 14,
                            //         vitality: 8,
                            //     },
                            //     // Add other classes with appropriate stat distributions
                            //     _ => Stats {
                            //         strength: 10,
                            //         agility: 10,
                            //         intelligence: 10,
                            //         vitality: 10,
                            //     },
                            // };
                            // stats.insert(player_entity, player_stats).unwrap();
                            // 
                            // // Resources (mana, stamina) based on class
                            // let (base_mana, base_stamina) = match class {
                            //     PlayerClass::Wizard | PlayerClass::Warlock => (20, 10),
                            //     PlayerClass::Druid | PlayerClass::Cleric => (15, 15),
                            //     PlayerClass::Ranger | PlayerClass::Paladin => (10, 15),
                            //     PlayerClass::Fighter | PlayerClass::Rogue => (5, 20),
                            //     PlayerClass::Monk => (10, 20),
                            // };
                            // resources
                            //     .insert(
                            //         player_entity,
                            //         Resource {
                            //             mana: base_mana,
                            //             max_mana: base_mana,
                            //             stamina: base_stamina,
                            //             max_stamina: base_stamina,
                            //         },
                            //     )
                            //     .unwrap();
                            // 
                            // // Experience starts at 0
                            // experiences
                            //     .insert(
                            //         player_entity,
                            //         Experience {
                            //             current: 0,
                            //             next_level: 100, // Base XP for level 2
                            //         },
                            //     )
                            //     .unwrap();
                            // 
                            // // Player entities use pathfinding to explore the dungeon
                            // movement_ais
                            //     .insert(
                            //         player_entity,
                            //         MovementAI {
                            //             kind: MovementAIKind::ExploreDungeon,
                            //         },
                            //     )
                            //     .unwrap();
                            // 
                            // // Player faction
                            // factions.insert(player_entity, Faction::Player).unwrap();
                            // 
                            // // Class-specific sprite
                            // let sprite_name =
                            //     format!("{}_sprite", class.to_string().to_lowercase());
                            // renderables
                            //     .insert(
                            //         player_entity,
                            //         Renderable {
                            //             texture_name: sprite_name,
                            //             i_w: 32, // Adjust sprite dimensions as needed
                            //             i_h: 32,
                            //             o_w: 32,
                            //             o_h: 32,
                            //             frame: 0,
                            //             total_frames: 4, // Assuming 4 animation frames
                            //         },
                            //     )
                            //     .unwrap();

                            // Store the player name in a resource or component
                            // This could be done via a PlayerName component or a map in a resource

                            // Change game state if needed
                            // *game_state = GameState::InDungeon;
                        }
                        // Other commands...
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
                    // Handle in-dungeon commands
                    // ...
                }
            }
        }
    }
}
