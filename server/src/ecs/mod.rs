pub mod systems;

use crate::commands::RpgCommand;
use crate::ecs::components::{MovementSpeed, Position, Renderable, TargetPosition};
use crate::ecs::systems::movement::Movement;
use crate::ecs::systems::random_wander::RandomWander;
use serde::Serialize;
use specs::{Builder, DispatcherBuilder, World, WorldExt};
use tokio::sync::broadcast::Sender;
use tokio::sync::mpsc::Receiver;
use tokio::sync::{broadcast, mpsc};

mod components;

mod entities;

pub struct GameState {
    ecs: World,
    rx: mpsc::Receiver<(String, RpgCommand)>,
    tx: broadcast::Sender<GameSnapShot>,
    // dispatcher: Dispatcher<'static, 'static>,
}

impl GameState {
    pub fn new(
        rx: mpsc::Receiver<(String, RpgCommand)>,
        tx: broadcast::Sender<GameSnapShot>,
    ) -> Self {
        let mut ecs = World::new();
        ecs.register::<Position>();
        // ecs.register::<Renderable>();
        // ecs.register::<Health>();

        Self { ecs, rx, tx }
    }
}

#[derive(Clone, Serialize)]
pub struct GameSnapShot {}

pub fn run_game_server(
    gamestate_sender: Sender<GameSnapShot>,
    commands_receiver: Receiver<(String, RpgCommand, bool)>,
) {
    let mut world = World::new();
    // register components
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<MovementSpeed>();
    world.register::<TargetPosition>();

    // build dispatcher with systems
    let mut dispatcher = DispatcherBuilder::new()
        .with(Movement, "movement", &[])
        .with(RandomWander, "idle", &[])
        .build();

    for i in 0..10 {
        world
            .create_entity()
            .with(Position { x: i, y: 0 })
            .with(MovementSpeed(2))
            .with(TargetPosition { x: 0, y: 0 })
            .build();
    }

    loop {
        // run systems
        dispatcher.dispatch(&mut world);

        // cleanup etc
        world.maintain();

        // sleep for a duration
        // TODO: figure out a time interval appropriate for this game
        std::thread::sleep(std::time::Duration::from_millis(1500));
    }
}

// match command {
//     RpgCommand::New(class) => {
// create player character with default values, store in persistence (player, class)
// }
// RpgCommand::Load => {
// load character from persistence (player)
// RpgCommand::Use(consumable) => {
//     // check if player has the consumable
// }
// RpgCommand::Buy(item) => {
//     // subtract player gold, player gets item
// }
// _ => unimplemented!(),
// }
// }
