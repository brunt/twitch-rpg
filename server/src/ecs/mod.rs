pub mod components;
pub mod systems;

use crate::commands::RpgCommand;
pub(crate) use crate::ecs::resources::DungeonExt;
use crate::ecs::systems::command_handler::{CommandHandlerSystem, CommandQueue};
use crate::ecs::systems::movement::Movement;
use crate::ecs::systems::random_wander::RandomWander;
use crate::ecs::systems::rendering::Rendering;
use crate::ecs::world::create_world;
use common::GameSnapShot;
use serde::Serialize;
use specs::{Builder, DispatcherBuilder, Join, World, WorldExt};
use tatami_dungeon::{Dungeon, GenerateDungeonParams};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{broadcast, mpsc};

mod entities;
pub mod resources;
mod shop;
mod world;

pub struct GameWorld {
    pub ecs: World,
    pub rx: mpsc::Receiver<(String, RpgCommand, bool)>,
}

impl GameWorld {
    pub fn new(rx: Receiver<(String, RpgCommand, bool)>) -> Self {
        let mut world = create_world();
        //TODO: panics on big dimensions
        // world.generate_dungeon( GenerateDungeonParams {
        //     num_floors: 3,
        //     dimensions: (100, 100),
        //     tiles_per_cell: 4,
        //     ..GenerateDungeonParams::default()
        // });
        world.generate_dungeon(GenerateDungeonParams::default());

        // ecs.generate_dungeon(GenerateDungeonParams::default());
        Self { ecs: world, rx }
    }
}

pub fn run_game_server(
    gamestate_sender: broadcast::Sender<GameSnapShot>,
    commands_receiver: Receiver<(String, RpgCommand, bool)>,
) {
    let mut gs = GameWorld::new(commands_receiver);

    gs.ecs.insert(CommandQueue::default());

    // build dispatcher with systems
    let mut dispatcher = DispatcherBuilder::new()
        .with(CommandHandlerSystem, "command_handler", &[])
        .with(Movement, "movement", &["command_handler"])
        .with(RandomWander, "idle", &["movement"])
        .with(
            Rendering {
                sender: gamestate_sender,
            },
            "rendering",
            &[],
        )
        .build();

    loop {
        while let Ok((player, command, is_privileged)) = gs.rx.try_recv() {
            if let Some(queue) = gs.ecs.get_mut::<CommandQueue>() {
                queue.push_back((player, command, is_privileged));
            }
        }

        // run systems
        dispatcher.dispatch(&mut gs.ecs);

        // cleanup etc
        gs.ecs.maintain();

        // let entites = gs.ecs.entities();
        // let classes = gs.ecs.read_storage::<CharacterClass>();
        // let levels = gs.ecs.read_storage::<Level>();
        // let monies = gs.ecs.read_storage::<Money>();
        // let healths = gs.ecs.read_storage::<HealthComponent>();
        // for (entity, class, level, money, health) in (&entites, &classes, &levels, &monies, &healths).join() {
        //     println!("{:?} {:?} {:?} {:?} {:?}", entity, class, level, money, health);
        // }

        // sleep for a duration
        // TODO: figure out a time interval appropriate for this game
        std::thread::sleep(std::time::Duration::from_millis(1500));
    }
}
