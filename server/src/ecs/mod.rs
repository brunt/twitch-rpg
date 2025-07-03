pub mod systems;
pub mod components;

use crate::commands::RpgCommand;
use crate::ecs::components::{MovementSpeed, Position, Renderable, TargetPosition};
use crate::ecs::systems::movement::Movement;
use crate::ecs::systems::random_wander::RandomWander;
use crate::ecs::systems::command_handler::{CommandHandlerSystem, CommandQueue};
use serde::Serialize;
use specs::{Builder, DispatcherBuilder, World, WorldExt};
use tatami_dungeon::{Dungeon, GenerateDungeonParams};
use tokio::sync::broadcast::Sender;
use tokio::sync::mpsc::Receiver;
use tokio::sync::{broadcast, mpsc};


mod entities;
mod world;
mod resources;

pub struct GameState {
    pub dungeon: Dungeon,
    pub ecs: World,
    pub rx: mpsc::Receiver<(String, RpgCommand, bool)>,
    pub tx: broadcast::Sender<GameSnapShot>,
    // dispatcher: Dispatcher<'static, 'static>,
}

impl GameState {
    pub fn new(
        rx: Receiver<(String, RpgCommand, bool)>,
        tx: broadcast::Sender<GameSnapShot>,
    ) -> Self {
        let mut ecs = World::new();
        let dungeon = Dungeon::generate_with_params(GenerateDungeonParams{
            num_floors: 3,
            dimensions: (40, 40),
            tiles_per_cell: 1,
                ..GenerateDungeonParams::default()
        });
        ecs.register::<Position>();
        ecs.register::<Renderable>();
        ecs.register::<MovementSpeed>();
        ecs.register::<TargetPosition>();
        // ecs.register::<Renderable>();
        // ecs.register::<Health>();

        Self { dungeon, ecs, rx, tx }
    }
}

#[derive(Clone, Serialize)]
pub struct GameSnapShot {}

pub fn run_game_server(
    gamestate_sender: Sender<GameSnapShot>,
    commands_receiver: Receiver<(String, RpgCommand, bool)>,
) {
    let mut gs = GameState::new(commands_receiver, gamestate_sender);


    gs.ecs.insert(CommandQueue::default());


    // build dispatcher with systems
    let mut dispatcher = DispatcherBuilder::new()
        .with(CommandHandlerSystem, "command_handler", &[])
        .with(Movement, "movement", &["command_handler"])
        .with(RandomWander, "idle", &["movement"])
        .build();


    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position { x: i, y: 0 })
            .with(MovementSpeed(2))
            .with(TargetPosition { x: 0, y: 0 })
            .build();
    }

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

        // sleep for a duration
        // TODO: figure out a time interval appropriate for this game
        std::thread::sleep(std::time::Duration::from_millis(1500));
    }
}

pub fn change_game_state(world: &mut World, new_state: resources::GameState) {
    if let Some(state) = world.get_mut::<resources::GameState>() {
        *state = new_state.clone();
        println!("Game state changed to: {:?}", new_state);
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
